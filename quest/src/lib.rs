#![no_std]
use gstd::{msg::{self, source}, prelude::*, debug, collections::BTreeMap, ActorId, exec};
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
            if !quests.approved_recruiters.contains(&msg::source()) {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Publish"), 
                    reason: String::from("Publisher is not approved."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 2. check if the quest is valid
            if !quest.is_complete() {
                let _ = msg::reply(QuestEvent::OperationErr { 
                    name: String::from("Publish"), 
                    reason: String::from("Quest is not complete."),
                    timestamp: exec::block_timestamp(),
                }, 0);
                return;
            }
            // 3. publish the quest
            let quest_id = quests.quest_id;
            quests.quest_id += 1;
            quests.num_counter += 1;
            quests.quests.insert(quest_id, quest);

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
            quest.seeker_submission.insert(msg::source(), String::new());
            // enter the seeker into the seeker status map
            quest.seeker_status.insert(msg::source(), Status::Claimed);

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
            quest.seeker_submission.insert(msg::source().clone(), submission);
            quest.seeker_status.insert(msg::source(), Status::WaitingReply);
            let _ = msg::reply(QuestEvent::OperationSuccess { 
                name: String::from("Submit"), 
                timestamp: exec::block_timestamp(), 
            }, 0);
        },
        _ => {},
    }

}