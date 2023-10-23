#![no_std]
use gstd::{msg, prelude::*, collections::BTreeMap, ActorId, exec};
use quest_io::*;

struct Quests {
	// the publisher of the quest contract
	owner: ActorId,
	// the address of the account's master contract
	account_contract_addr: Option<ActorId>,
    // the list of approved recruiters
    approved_recruiters: Vec<ActorId>,
	// maximum number of quests a single quest storage contract should handle
	max_num_quests: u32,
	// current number of quests in this contract
	num_counter: u32,
    // TODO: ideally, this should be a hash over the quest content
    // now it's just a counter that will only increase when a new quest is published
    quest_id: u32,
	// u32 represents quest id
	quests: BTreeMap<u32, Quest>,
}

impl Quests {
    // check if a recruiter is approved
    pub fn is_approved(&self, recruiter: &ActorId) -> bool {
        self.approved_recruiters.contains(recruiter)
    }
}

static mut CONTRACT: Option<Quests> = None;

#[no_mangle]
extern "C" fn init() {
    let _init_info: String = msg::load().expect("Failed to load init info");
    unsafe {
        CONTRACT = Some(Quests {
            owner: msg::source(),
            account_contract_addr: None,
            approved_recruiters: Vec::new(),
            max_num_quests: 1000,
            num_counter: 0,
            quest_id: 0,
            quests: BTreeMap::new(),
        });
    }

    let _ = msg::reply(QuestEvent::ContractInitiated, 0);
}

#[no_mangle]
extern "C" fn handle() {
    let action: QuestAction = msg::load().expect("Failed to load action");
    let quests: &mut Quests = unsafe { CONTRACT.as_mut().expect("Quest contract not initialized.") };

    match action {
        QuestAction::Publish { quest } => {
            // 0. check if the max number of quests is reached
            if quests.num_counter >= quests.max_num_quests {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Publish"), 
                    reason: String::from("Max number of quests reached."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 1. check if the publisher is approved
            if !quests.is_approved(&msg::source()) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Publish"), 
                    reason: String::from("Publisher is not approved."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 2. check if the quest is valid
            
            // 3. publish the quest
            let quest_id = quests.quest_id;
            quests.quest_id += 1;
            quests.num_counter += 1;
            let new_quest = Quest {
                id: quest_id,
                publisher: msg::source(),
                status: QuestStatus::Open,
                title: quest.title,
                position: quest.position,
                deadline: quest.deadline,
                img: quest.img,
                deliverables: quest.deliverables,
                details: quest.details,
                seeker_status: BTreeMap::new(),
                seeker_submission: BTreeMap::new(),
            };
            quests.quests.insert(quest_id, new_quest);

            let _ = msg::reply(QuestEvent::OperationSuccess { 
                name: String::from("Publish"), 
                timestamp: exec::block_timestamp(),
            }, 0);
        },
        QuestAction::Claim { quest_id } => {
            // 1. check if the quest exists
            if !quests.quests.contains_key(&quest_id) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Claim"), 
                    reason: String::from("Quest does not exist."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 2. check if the quest is open
            if !quests.quests.get(&quest_id).unwrap().is_open() {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Claim"), 
                    reason: String::from("Quest is not open."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 3. claim the quest
            let quest = quests.quests.get_mut(&quest_id).unwrap();
            // create an empty submission for the seeker
            quest.add_submission(msg::source().clone(), String::new());
            // enter the seeker into the seeker status map
            quest.change_seeker_status(msg::source(), Status::Claimed);

            let _ = msg::reply(QuestEvent::OperationSuccess { 
                name: String::from("Claim"), 
                timestamp: exec::block_timestamp(), 
            }, 0);
        },
        QuestAction::Submit { quest_id, submission } => {
            // 1. check if the quest exists
            if !quests.quests.contains_key(&quest_id) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Submit"), 
                    reason: String::from("Quest does not exist."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }

            let quest = quests.quests.get_mut(&quest_id).unwrap();

            // 2. check if the quest is NOT closed
            if quest.is_closed() {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Submit"), 
                    reason: String::from("Quest is closed."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 3. seekers can only submit if their current status is claimed,
            // this means the seeker has already claimed the quest but has not submitted yet
            if !quest.seeker_status_match(&msg::source(), Status::Claimed) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Submit"), 
                    reason: String::from("Seeker is either not claimed this quest or has already submitted."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 4. submit the quest
            quest.add_submission(msg::source().clone(), submission);
            quest.change_seeker_status(msg::source(), Status::WaitingReply);
            let _ = msg::reply(QuestEvent::OperationSuccess { 
                name: String::from("Submit"), 
                timestamp: exec::block_timestamp(), 
            }, 0);
        },
        QuestAction::Interview { seeker, quest_id } => {
            // 1. check if the msg sender is an approved recruiter
            if !quests.is_approved(&msg::source()) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Interview"), 
                    reason: String::from("Recruiter is not approved."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 2. check if the quest exists
            if !quests.quests.contains_key(&quest_id) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Interview"), 
                    reason: String::from("Quest does not exist."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 3. check if the seeker has submitted to the quest
            if !quests.quests.get(&quest_id).unwrap().seeker_status_match(&seeker, Status::WaitingReply) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Interview"), 
                    reason: String::from("Seeker has not submitted to the quest."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 4. send interview invitation to the seeker
            let quest = quests.quests.get_mut(&quest_id).unwrap();
            quest.change_seeker_status(seeker, Status::InterviewReceived);
            let _ = msg::reply(QuestEvent::OperationSuccess { 
                name: String::from("Interview"), 
                timestamp: exec::block_timestamp(), 
            }, 0);
        },
        QuestAction::AcceptInterview { quest_id } => {
            // 1. check if the quest exists
            if !quests.quests.contains_key(&quest_id) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("AcceptInterview"), 
                    reason: String::from("Quest does not exist."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 2. check if the seeker has received an interview invitation
            if !quests.quests.get(&quest_id).unwrap().seeker_status_match(&msg::source(), Status::InterviewReceived) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("AcceptInterview"), 
                    reason: String::from("Seeker has not received an interview invitation."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 3. accept the interview invitation
            let quest = quests.quests.get_mut(&quest_id).unwrap();
            quest.change_seeker_status(msg::source(), Status::InterviewAccepted);
            let _ = msg::reply(QuestEvent::OperationSuccess { 
                name: String::from("AcceptInterview"), 
                timestamp: exec::block_timestamp(), 
            }, 0);
        },
        QuestAction::Offer { seeker, quest_id } => {
            // 1. check if the msg sender is an approved recruiter
            if !quests.is_approved(&msg::source()) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Offer"), 
                    reason: String::from("Recruiter is not approved."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 2. check if the quest exists
            if !quests.quests.contains_key(&quest_id) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Offer"), 
                    reason: String::from("Quest does not exist."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 3. check if the seeker has accepted an interview invitation
            if !quests.quests.get(&quest_id).unwrap().seeker_status_match(&seeker, Status::InterviewAccepted) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Offer"), 
                    reason: String::from("Seeker has not accepted an interview invitation."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 4. send offer to the seeker
            let quest = quests.quests.get_mut(&quest_id).unwrap();
            quest.change_seeker_status(seeker, Status::OfferReceived);
            let _ = msg::reply(QuestEvent::OperationSuccess { 
                name: String::from("Offer"), 
                timestamp: exec::block_timestamp(), 
            }, 0);
        },
        QuestAction::AcceptOffer { quest_id } => {
            // 1. check if the quest exists
            if !quests.quests.contains_key(&quest_id) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("AcceptOffer"), 
                    reason: String::from("Quest does not exist."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 2. check if the seeker has received an offer
            if !quests.quests.get(&quest_id).unwrap().seeker_status_match(&msg::source(), Status::OfferReceived) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("AcceptOffer"), 
                    reason: String::from("Seeker has not received an offer."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 3. accept the offer
            let quest = quests.quests.get_mut(&quest_id).unwrap();
            quest.change_seeker_status(msg::source(), Status::OfferAccepted);
            let _ = msg::reply(QuestEvent::OperationSuccess { 
                name: String::from("AcceptOffer"), 
                timestamp: exec::block_timestamp(), 
            }, 0);
        },
        QuestAction::Reject { seeker, quest_id } => {
            // 1. check if the msg sender is an approved recruiter
            if !quests.is_approved(&msg::source()) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Reject"), 
                    reason: String::from("Recruiter is not approved."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 2. check if the quest exists
            if !quests.quests.contains_key(&quest_id) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Reject"), 
                    reason: String::from("Quest does not exist."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }

            let quest = quests.quests.get_mut(&quest_id).unwrap();
            // 3. check if the seeker has either submitted or interviewed
            if !quest.seeker_status_match(&seeker, Status::WaitingReply) && !quest.seeker_status_match(&seeker, Status::InterviewAccepted) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Reject"), 
                    reason: String::from("Seeker has not submitted or interviewed."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 4. reject the seeker
            quest.change_seeker_status(seeker, Status::Rejected);
            let _ = msg::reply(QuestEvent::OperationSuccess { 
                name: String::from("Reject"), 
                timestamp: exec::block_timestamp(), 
            }, 0);
        },
        QuestAction::AddRecruiter { recruiter } => {
            // 1. check if the msg sender is the owner of the quest contract
            if msg::source() != quests.owner {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("AddRecruiter"), 
                    reason: String::from("Sender is not the owner of the quest contract."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 2. check if the recruiter is already in the approved recruiters list
            if quests.is_approved(&recruiter) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("AddRecruiter"), 
                    reason: String::from("Recruiter is already in the approved recruiters list."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 3. add the recruiter to the approved recruiters list
            quests.approved_recruiters.push(recruiter);
            let _ = msg::reply(QuestEvent::OperationSuccess { 
                name: String::from("AddRecruiter"), 
                timestamp: exec::block_timestamp(), 
            }, 0);
        },
        QuestAction::ChangeAccountContract { new_account_contract } => {
            // 1. check if the msg sender is the owner of the quest contract
            if msg::source() != quests.owner {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("ChangeAccountContract"), 
                    reason: String::from("Sender is not the owner of the quest contract."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 2. change the account contract address
            quests.account_contract_addr = Some(new_account_contract);
            let _ = msg::reply(QuestEvent::OperationSuccess { 
                name: String::from("ChangeAccountContract"), 
                timestamp: exec::block_timestamp(), 
            }, 0);
        }
        _ => {},
    }

}