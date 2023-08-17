#![no_std]
use gstd::{msg, prelude::*, debug};
use quest_io::{Quests, QuestAction};
use account_io::QuestId;

static mut QUESTS: Option<Quests> = None;

#[no_mangle]
extern "C" fn init() {
    let _init_info: String = msg::load().expect("Failed to load init message");

    unsafe {
        QUESTS = Some(Quests { 
            quests: HashMap::new(), 
            claimers_quests: HashMap::new(), 
        });
    }

    debug!("Quest contract initialized!");

    msg::reply("Quest contract initialized!", 0).expect("Failed to reply init");
}

#[no_mangle]
extern "C" fn handle() {
    let action: QuestAction = msg::load().expect("Failed to load quest action");
    let quests = unsafe { QUESTS.as_mut().expect("Quest not initialized") };

    match action {
        // TODO: need much stricter access control

        QuestAction::Publish { quest } => {
            // 1. generate a quest id
            // TODO: for now we just use a hardcoded id, need to have a random id generator in the future
            let quest_id = QuestId::new("01234567890123456789".to_string());
            // 2. store the quest in the quests map
            let publish_event = quests.publish(quest, quest_id);
            // 3. emit a QuestPublished event
            msg::reply(publish_event, 0).expect("Failed to reply publish event");
        },
        QuestAction::Claim { quest_id } => {
            let claim_event = quests.claim(msg::source(), quest_id);
            msg::reply(claim_event, 0).expect("Failed to reply claim event");
        },
        QuestAction::Submit { quest_id, submission } => {
            let submit_event = quests.submit(msg::source(), quest_id, submission);
            msg::reply(submit_event, 0).expect("Failed to reply submit event");
        },
        QuestAction::Grade { qeust_id, claimer, grades } => {
            let grade_event = quests.grade(claimer, qeust_id, grades);
            msg::reply(grade_event, 0).expect("Failed to reply grade event");
        },
    }
}