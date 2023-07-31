#![no_std]
use gstd::{msg, prelude::*, debug};
use quest_io::{Quest, QuestAction, QuestEvent};

static mut QUEST: Option<Quest> = None;

#[derive(Decode)]
#[codec(crate = gstd::codec)]
pub struct QuestInfo {
    pub field: String,
}

#[no_mangle]
extern "C" fn init() {
    // 1. load quest information from message received from the factory contract
    let _quest_info_json: QuestInfo = msg::load().expect("Failed to load quest information");
    // 2. parse the information into a shadow quest struct
    // 3. copy info into actual quest struct
    unsafe {
        QUEST = Some(Quest {
            id: 0.into(),
            owner: 2.into(),
            name: String::from("initial quest"),
            description: String::from("initial quest description"),
            deadline: 0,
            claimers: Vec::new(),
            claimer_submit: HashMap::new(),
            claimer_grade: HashMap::new(),
        });
    }

    debug!("Quest initialized");

    msg::reply(String::from("Quest Created!"), 0).expect("Quest Creation Failed");
}

#[no_mangle]
extern "C" fn handle() {
    let action: QuestAction = msg::load().expect("Failed to load quest action");
    let quest = unsafe { QUEST.as_mut().expect("Quest not initialized") };

    match action {
        QuestAction::Claim => {
            match quest.claim(msg::source()) {
                QuestEvent::Claimed => {
                    msg::reply(QuestEvent::Claimed, 0).expect("Failed to emit claim event");
                },
                QuestEvent::ErrorClaimerExists => {
                    msg::reply(QuestEvent::ErrorClaimerExists, 0).expect("Failed to emit claim error event");
                },
                _ => {
                    debug!("Unknown error");
                }
            }
        },
        QuestAction::Submit(s) => {
            match quest.submit(msg::source(), s) {
                QuestEvent::Submitted => {
                    msg::reply(QuestEvent::Submitted, 0).expect("Failed to emit submit event");
                },
                QuestEvent::ErrorSubmitterNotExists => {
                    msg::reply(QuestEvent::ErrorSubmitterNotExists, 0).expect("Failed to emit submit error event");
                },
                _ => {
                    debug!("Unknown error");
                }
            }
        },
        QuestAction::Grade(recipient, grades) => {
            match quest.grade(msg::source(), recipient, grades) {
                QuestEvent::Graded => {
                    msg::reply(QuestEvent::Graded, 0).expect("Failed to emit grade event");
                },
                QuestEvent::ErrorNotQuestOwner => {
                    msg::reply(QuestEvent::ErrorNotQuestOwner, 0).expect("Failed to emit grade error event");
                },
                QuestEvent::ErrorSubmitterNotExists => {
                    msg::reply(QuestEvent::ErrorSubmitterNotExists, 0).expect("Failed to emit grade error event");
                },
                _ => {
                    debug!("Unknown error");
                }
            }
        },
    }
}

// #[no_mangle]
// extern "C" fn metahash() {
//     let metahash: [u8; 32] = include!("../.metahash");
//     msg::reply(metahash, 0)
//         .expect("Failed to share metahash");
// }
