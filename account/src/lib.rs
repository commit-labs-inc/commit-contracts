#![no_std]
use gstd::{msg, prelude::*, debug, ActorId, collections::{HashMap, HashSet}, exec, String};
use account_io::*;

#[derive(Default)]
struct Accounts {
	// the wallet address that deployed this contract
	owner_id: ActorId,
	// the maximum number of accounts this contract should handle
	max_num_accounts: u32,
	// current number of accounts
	num_counter: u32,
	// Currently, one address can only have one role.
	// accounts that have the seeker's role
	seekers: HashSet<ActorId>,
	// accounts that have the recruiter's role
	recruiters: HashSet<ActorId>,
	// map wallet address to Account
	accounts: HashMap<ActorId, Account>,
}

static mut CONTRACT: Option<Accounts> = None;

#[no_mangle]
extern "C" fn init() {
    let _init_info: String = msg::load().expect("Failed to load init info");
    unsafe {
        CONTRACT = Some(Accounts {
            owner_id: msg::source(),
            max_num_accounts: 10000,
            num_counter: 0,
            ..Default::default()
        });
    }

    debug!("account contract initialized!");

    let _ = msg::reply(AccountEvent::ContractInitiated, 0);
}

#[no_mangle]
extern "C" fn handle() {
    let action: AccountAction = msg::load().expect("Failed to load action");
    let accounts: &mut Accounts = unsafe { CONTRACT.as_mut().expect("Account not initialized") };

    match action {
        AccountAction::Login { role } => {

            match role {
                Roles::Seeker => {
                    if accounts.is_exists(msg::source(), Roles::Seeker) {
                        let found_account = accounts.accounts.get(&msg::source()).unwrap().clone();
                        let _ = msg::reply(AccountEvent::AccountExists { 
                            username: found_account.username.clone(),
                         }, 0);
                    } else {
                        // This situation should never happen after switching to a three-tiered contract structure for account contract.
                        if !accounts.check_capacity() {
                            let _ = msg::reply(AccountEvent::Err { msg: String::from("The maximum number of accounts has been reached.") }, 0);
                            return;
                        }
                        // insert the address into the seekers set
                        accounts.seekers.insert(msg::source());
                        // create a new account
                        let new_account = Account {
                            username: String::from(""),
                            role: Roles::Seeker,
                            badges: Vec::new(),
                            quests: Vec::new(),
                            quest_decks: Vec::new(),
                        };
                        // insert the new account into the accounts map
                        accounts.accounts.insert(msg::source(), new_account);
                        // incresase the counter
                        accounts.num_counter += 1;

                        let _ = msg::reply(AccountEvent::AccountCreated { account: msg::source(), timestamp: exec::block_timestamp() }, 0);
                    }
                },
                Roles::Recruiter => {
                    if accounts.is_exists(msg::source(), Roles::Recruiter) {
                        let found_account = accounts.accounts.get(&msg::source()).unwrap().clone();
                        let _ = msg::reply(AccountEvent::AccountExists { 
                            username: found_account.username.clone(),
                         }, 0);
                    } else {
                        // This situation should never happen after switching to a three-tiered contract structure for account contract.
                        if !accounts.check_capacity() {
                            let _ = msg::reply(AccountEvent::Err { msg: String::from("The maximum number of accounts has been reached.") }, 0);
                            return;
                        }
                        // insert the address into the recruiters set
                        accounts.recruiters.insert(msg::source());
                        // create a new account
                        let new_account = Account {
                            username: String::from(""),
                            role: Roles::Recruiter,
                            badges: Vec::new(),
                            quests: Vec::new(),
                            quest_decks: Vec::new(),
                        };
                        // insert the new account into the accounts map
                        accounts.accounts.insert(msg::source(), new_account);
                        // incresase the counter
                        accounts.num_counter += 1;

                        let _ = msg::reply(AccountEvent::AccountCreated { account: msg::source(), timestamp: exec::block_timestamp() }, 0);
                    }
                }
            }
        },
        AccountAction::ChangeName { new_name } => {
            if accounts.change_name(new_name) {
                let _ = msg::reply(AccountEvent::NameChanged { account: msg::source(), timestamp: exec::block_timestamp() }, 0);
            } else {
                let _ = msg::reply(AccountEvent::Err { msg: String::from("Name changing failed") }, 0);
            }
        },
        AccountAction::Delete => {
            if accounts.delete_account() {
                // decrease the counter until 0
                if accounts.num_counter > 0 {
                    accounts.num_counter -= 1;
                }

                let _ = msg::reply(AccountEvent::AccountDeleted { account: msg::source(), timestamp: exec::block_timestamp() }, 0);
            } else {
                let _ = msg::reply(AccountEvent::Err { msg: String::from("Delete account failed") }, 0);
            }
        },
        // This action will trigger the status change of a seeker's quest from submitted to interview received.
        AccountAction::SendInterview { quest_id, seeker_id } => {
            // 1. check the current status = submitted
            let curr_status = accounts.get_status(seeker_id.clone(), quest_id.clone());
            if *curr_status != Status::Seeker(SeekerStatus::Submitted) {
                let _ = msg::reply(AccountEvent::Err { msg: String::from("Only seekers who have submitted their applications can get interviews.") }, 0);
                return;
            }
            // 2. change the status to interview received
            accounts.receive_interview(quest_id.clone(), seeker_id);
            let _ = msg::reply(AccountEvent::InterviewReceived { quest_id, seeker_id }, 0);
        },
        // This action will trigger the status change of a seeker's quest from interview received to offer received.
        AccountAction::SendOffer { quest_id, recruiter_id, seeker_id } => {
            // 1. check the current status = interview accepted
            let curr_status = accounts.get_status(seeker_id.clone(), quest_id.clone());
            if *curr_status != Status::Seeker(SeekerStatus::InterviewAccepted) {
                let _ = msg::reply(AccountEvent::Err { msg: String::from("Only seekers who have interviews accepted can get offers.") }, 0);
                return;
            }
            // 2. change the status to offer received
            accounts.receive_offer(quest_id.clone(), seeker_id);
            let _ = msg::reply(AccountEvent::OfferReceived { quest_id, recruiter_id, seeker_id }, 0);
        },
        // This action will trigger the status change of a seeker's quest from interview received to accepted.
        AccountAction::AcceptInterview { quest_id, seeker_id } => {
            // 1. check the current status = interview received
            let curr_status = accounts.get_status(seeker_id.clone(), quest_id.clone());
            if *curr_status != Status::Seeker(SeekerStatus::InterviewReceived) {
                let _ = msg::reply(AccountEvent::Err { msg: String::from("Only seekers who have received interviews can accept interviews.") }, 0);
                return;
            }
            // 2. change the status to interview accepted
            accounts.accept_interview(quest_id.clone(), seeker_id);
            let _ = msg::reply(AccountEvent::InterviewAccepted { quest_id, seeker_id }, 0);
        },
        // This action will trigger the status change of a seeker's quest from offer received to accepted.
        AccountAction::AcceptOffer { quest_id, seeker_id } => {
            // 1. check the current status = offer received
            let curr_status = accounts.get_status(seeker_id.clone(), quest_id.clone());
            if *curr_status != Status::Seeker(SeekerStatus::OfferReceived) {
                let _ = msg::reply(AccountEvent::Err { msg: String::from("Only seekers who have received offers can accept offers.") }, 0);
                return;
            }
            // 2. change the status to offer accepted
            accounts.accept_offer(quest_id.clone(), seeker_id);
            let _ = msg::reply(AccountEvent::OfferAccepted { quest_id, seeker_id }, 0);
        },
        AccountAction::RecruiterReject { quest_id, seeker_id } => {
            // 1. check the seeker's current status = submitted or interview accepted
            let curr_status = accounts.get_status(seeker_id.clone(), quest_id.clone());
            if *curr_status != Status::Seeker(SeekerStatus::Submitted) && *curr_status != Status::Seeker(SeekerStatus::InterviewAccepted) {
                let _ = msg::reply(AccountEvent::Err { msg: String::from("Only seekers who have submitted their applications or have accepted interviews can be rejected.") }, 0);
                return;
            }
            // 2. change the status to rejected
            accounts.recruiter_reject(quest_id.clone(), seeker_id);
            let _ = msg::reply(AccountEvent::Rejected { quest_id, seeker_id }, 0);
        },
        AccountAction::PublishQuest { recruiter_id, quest_id } => {
            accounts.accounts.get_mut(&recruiter_id).unwrap().quests.push((quest_id.clone(), Status::Recruiter(RecruiterStatus::Published)));
            let _ = msg::reply(AccountEvent::QuestPublished { recruiter_id, quest_id, timestamp: exec::block_timestamp() }, 0);
        }
    }
}

impl Accounts {
    // TODO: change the return type to Result.
    fn change_name(&mut self, new_name: String) -> bool {
        if let Some(account) = self.accounts.get_mut(&msg::source()) {
            account.username = new_name;
            true
        } else {
            // if no record found, means the account does not exist
            false
        }
    }
    // delete the account mapped to the sender's address
    fn delete_account(&mut self) -> bool {
        if let Some(_) = self.accounts.get_mut(&msg::source()) {
            self.accounts.remove(&msg::source());
            true
        } else {
            // if no record found, means the account does not exist
            false
        }
    }
    // TODO: in the future when internal messaging system is open,
    // we will add more functionalities to this function.
    fn receive_interview(&mut self, quest_id: u32, seeker_id: ActorId) {
        if let Some(account) = self.accounts.get_mut(&seeker_id) {
            for (id, status) in &mut account.quests {
                if id == &quest_id {
                    *status = Status::Seeker(SeekerStatus::InterviewReceived);
                }
            }
        }
    }
    // TODO: in the future when internal messaging system is open,
    // we will add more functionalities to this function.
    fn receive_offer(&mut self, quest_id: u32, seeker_id: ActorId) {
        if let Some(account) = self.accounts.get_mut(&seeker_id) {
            for (id, status) in &mut account.quests {
                if id == &quest_id {
                    *status = Status::Seeker(SeekerStatus::OfferReceived);
                }
            }
        }
    }
    // TODO: in the future when internal messaging system is open,
    // we will add more functionalities to this function.
    fn accept_interview(&mut self, quest_id: u32, seeker_id: ActorId) {
        if let Some(account) = self.accounts.get_mut(&seeker_id) {
            for (id, status) in &mut account.quests {
                if id == &quest_id {
                    *status = Status::Seeker(SeekerStatus::InterviewAccepted);
                }
            }
        }
    }

    fn accept_offer(&mut self, quest_id: u32, seeker_id: ActorId) {
        if let Some(account) = self.accounts.get_mut(&seeker_id) {
            for (id, status) in &mut account.quests {
                if id == &quest_id {
                    *status = Status::Seeker(SeekerStatus::OfferAccepted);
                }
            }
        }
    }

    fn recruiter_reject(&mut self, quest_id: u32, seeker_id: ActorId) {
        if let Some(account) = self.accounts.get_mut(&seeker_id) {
            for (id, status) in &mut account.quests {
                if id == &quest_id {
                    *status = Status::Seeker(SeekerStatus::Rejected);
                }
            }
        }
    }

    // check if an address has been already registered as role.
    fn is_exists(&self, address: ActorId, role: Roles) -> bool {
        match role {
            Roles::Seeker => {
                self.seekers.contains(&address)
            },
            Roles::Recruiter => {
                self.recruiters.contains(&address)
            }
        }
    }
    // return an address's quest status
    fn get_status(&self, address: ActorId, quest_id: u32) -> &Status {
        if let Some(account) = self.accounts.get(&address) {
            for (id, status) in &account.quests {
                if id == &quest_id {
                    return status.clone();
                }
            }
        }
        &Status::Seeker(SeekerStatus::None)
    }
    // returns true if there are still capacity for new accounts
    fn check_capacity(&self) -> bool {
        self.num_counter < self.max_num_accounts
    }
}