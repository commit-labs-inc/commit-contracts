#![no_std]
use gstd::{msg, prelude::*, debug};
use quest_io::{Quest, Quests, QuestAction, QuestEvent};

static mut QUESTS: Option<Quests> = None;

/* #[derive(Decode)]
#[codec(crate = gstd::codec)]
pub struct QuestInfo {
    pub field: String,
} */

#[no_mangle]
extern "C" fn init() {
    let _init_info: String = msg::load().expect("Failed to load init info");
    // For now, we'll initialize a quest map with a single quest for testing
    let quest = Quest {
        owner: msg::source(),
        name: String::from("Test Quest"),
        description: String::from("This is a test quest"),
        deadline: 0,
        claimers: Vec::new(),
        claimer_submit: HashMap::new(),
        claimer_grade: HashMap::new(),
    };

    let mut map = HashMap::new();
    // TODO: quest id should be generated from the hash of the quest in the future
    map.insert(String::from("a fake quest id for testing only"), quest);

    unsafe {
        QUESTS = Some(Quests {
            map,
        });
    }

    debug!("Quests initialized");

    msg::reply(String::from("Quests Map Created!"), 0).expect("Quests Creation Failed");
}

#[no_mangle]
extern "C" fn handle() {
    let action: QuestAction = msg::load().expect("Failed to load quest action");
    let quests = unsafe { QUESTS.as_mut().expect("Quest not initialized") };

    match action {
        QuestAction::Claim(quest_id) => {
            // we first index a quest with the quest id received from the action,
            // then we add the claimer to the quest
            // this assumes the claimer is calling this contract directly, without orchestrator in between
            match quests.map.get_mut(&quest_id) {
                Some(quest) => {
                    match quest.claim(msg::source()) {
                        QuestEvent::Claimed => {
                            debug!("current claimer list is: {:?}", quest.claimers);
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
                None => {
                    debug!("Quest not found");
                    msg::reply(QuestEvent::UnknownError, 0).expect("Failed to emit unknown error event");
                }
            }
        },
        QuestAction::Submit(quest_id, submission) => {
            match quests.map.get_mut(&quest_id) {
                Some(quest) => {
                    match quest.submit(msg::source(), submission) {
                        QuestEvent::Submitted => {
                            debug!("current submission list is: {:?}", quest.claimer_submit);
                            msg::reply(QuestEvent::Submitted, 0).expect("Failed to emit submit event");
                        },
                        QuestEvent::ErrorSubmitterNotExists => {
                            debug!("Submitter not found");
                            msg::reply(QuestEvent::ErrorSubmitterNotExists, 0).expect("Failed to emit submit error event");
                        },
                        QuestEvent::ErrorAlreadySubmitted => {
                            debug!("Already submitted");
                            msg::reply(QuestEvent::ErrorAlreadySubmitted, 0).expect("Failed to emit submit error event");
                        },
                        QuestEvent::ErrorDeadlinePassed => {
                            debug!("Deadline passed");
                            msg::reply(QuestEvent::ErrorDeadlinePassed, 0).expect("Failed to emit submit error event");
                        }
                        _ => {
                            debug!("Unknown error");
                        }
                    }
                },
                None => {
                    debug!("Quest not found");
                    msg::reply(QuestEvent::UnknownError, 0).expect("Failed to emit unknown error event");
                }
            }
        },
        /*
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
        }, */
    }
}

// #[no_mangle]
// extern "C" fn metahash() {
//     let metahash: [u8; 32] = include!("../.metahash");
//     msg::reply(metahash, 0)
//         .expect("Failed to share metahash");
// }
