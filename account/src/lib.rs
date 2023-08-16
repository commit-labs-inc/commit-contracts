#![no_std]
use gstd::{msg, prelude::*, debug};
use account_io::{AccountAction, AccountEvent, Accounts, Profile, SeekerProfile, RecruiterProfile, Status, QuestId};

static mut ACCOUNTS: Option<Accounts> = None;

#[no_mangle]
extern "C" fn init() {
    let _init_info: String = msg::load().expect("Failed to load init message");

    unsafe {
        ACCOUNTS = Some(Accounts {
            seeker_accounts: HashMap::new(),
            recruiter_accounts: HashMap::new(),
        });
    }

    debug!("account contract initialized!");

    msg::reply(String::from("Account contract initialized!"), 0).expect("Failed to reply init message");
}

#[no_mangle]
extern "C" fn handle() {
    let action: AccountAction = msg::load().expect("Failed to load action message");
    let account = unsafe { ACCOUNTS.as_mut().unwrap() };

    match action {
        AccountAction::Login { role } => {
            // 1. Search if the account exists
            if let Ok(profile) = account.search(msg::source()) {
            // 2. If exists, send relevant info to sender's mailbox
                match profile {
                    // We seperate into two branches here so in the future we can add specific logic for each role
                    Profile::Seeker(seeker_profile) => {
                        msg::reply(AccountEvent::LoginSuccess { profile: Profile::Seeker(seeker_profile) }, 0).expect("send seeker profile reply failed.");
                        debug!("seeker profile sent");
                    },
                    Profile::Recruiter(recruiter_profile) => {
                        msg::reply(AccountEvent::LoginSuccess { profile: Profile::Recruiter(recruiter_profile) }, 0).expect("send recruiter profile reply failed.");
                        debug!("recruiter profile sent");
                    }
                }
            } else {
            // 3. If not, create a new account and send relevant info to sender's mailbox
                match role.as_str() {
                    "seeker" => {
                        let seeker_profile = SeekerProfile::default();
                        account.seeker_accounts.insert(msg::source(), seeker_profile.clone());
                        debug!("seeker profile created");
                        msg::reply(AccountEvent::LoginSuccess { profile: Profile::Seeker(seeker_profile) }, 0).expect("send seeker profile reply failed.");
                    },
                    "provider" => {
                        let recruiter_profile = RecruiterProfile::default();
                        account.recruiter_accounts.insert(msg::source(), recruiter_profile.clone());
                        debug!("recruiter profile created");
                        msg::reply(AccountEvent::LoginSuccess { profile: Profile::Recruiter(recruiter_profile) }, 0).expect("send seeker profile reply failed.");
                    },
                    _ => {
                        panic!("Invalid role");
                    }
                }
            }
        },
        AccountAction::Record { subject, action, quest_id } => {
            // TODO: need to first check the message sender is the quest contract
            match action.as_str() {
                "publish" => {
                    // TODO: need to check if 1) recruiter exists

                    if let Some(recruiter_profile) = account.recruiter_accounts.get_mut(&subject) {
                        recruiter_profile.publish(QuestId::new(quest_id), Status::Open);
                        debug!("quest published");
                        msg::reply(AccountEvent::RecordSuccess, 0).expect("send record success reply failed.");
                    } else {
                        debug!("recruiter profile not found!");
                        msg::reply(AccountEvent::RecordFailed, 0).expect("send record failed reply failed.");
                    }
                },
                "claim" => {
                    // TODO: need to check if 1) seeker exists 2) quest exists 3) quest is open

                    if let Some(seeker_profile) = account.seeker_accounts.get_mut(&subject) {
                        seeker_profile.claim(QuestId::new(quest_id), Status::InProgress);
                        debug!("quest claimed");
                        msg::reply(AccountEvent::RecordSuccess, 0).expect("send record success reply failed.");
                    } else {
                        debug!("seeker profile not found!");
                        msg::reply(AccountEvent::RecordFailed, 0).expect("send record failed reply failed.");
                    };
                },
                "submit" => {
                    // TODO: need to check if: 1) seeker exists 2) quest exists 3) quest is claimed by seeker

                    if let Some(seeker_profile) = account.seeker_accounts.get_mut(&subject) {
                        seeker_profile.update_status(&QuestId::new(quest_id), Status::Submitted);
                        debug!("quest submitted");
                        msg::reply(AccountEvent::RecordSuccess, 0).expect("send record success reply failed.");
                    } else {
                        debug!("seeker profile not found!");
                        msg::reply(AccountEvent::RecordFailed, 0).expect("send record failed reply failed.");
                    };
                },
                "grade_accepted" => {
                    // TODO: need to check if: 1) recruiter exists 2) quest exists 3) quest is submitted by seeker

                    if let Some(seeker_profile) = account.seeker_accounts.get_mut(&subject) {
                        seeker_profile.update_status(&QuestId::new(quest_id), Status::Accepted);
                        debug!("quest graded: Accepted!");
                        msg::reply(AccountEvent::RecordSuccess, 0).expect("send record success reply failed.");
                    } else {
                        debug!("seeker profile not found!");
                        msg::reply(AccountEvent::RecordFailed, 0).expect("send record failed reply failed.");
                    };
                },
                "grade_generally_good" => {
                    // TODO: need to check if: 1) recruiter exists 2) quest exists 3) quest is submitted by seeker

                    if let Some(seeker_profile) = account.seeker_accounts.get_mut(&subject) {
                        seeker_profile.update_status(&QuestId::new(quest_id), Status::GenerallyGood);
                        debug!("quest graded: GenerallyGood!");
                        msg::reply(AccountEvent::RecordSuccess, 0).expect("send record success reply failed.");
                    } else {
                        debug!("seeker profile not found!");
                        msg::reply(AccountEvent::RecordFailed, 0).expect("send record failed reply failed.");
                    };
                },
                "grade_need_improvements" => {
                    // TODO: need to check if: 1) recruiter exists 2) quest exists 3) quest is submitted by seeker

                    if let Some(seeker_profile) = account.seeker_accounts.get_mut(&subject) {
                        seeker_profile.update_status(&QuestId::new(quest_id), Status::NeedImprovements);
                        debug!("quest graded: Need Improvements!");
                        msg::reply(AccountEvent::RecordSuccess, 0).expect("send record success reply failed.");
                    } else {
                        debug!("seeker profile not found!");
                        msg::reply(AccountEvent::RecordFailed, 0).expect("send record failed reply failed.");
                    };
                },
                _ => {
                    msg::reply(AccountEvent::RecordFailed, 0).expect("send record failed reply failed.");
                }
            }
        }
    }
}