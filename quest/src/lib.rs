#![no_std]
use gstd::{msg, prelude::*, collections::BTreeMap, ActorId};
use quest_io::*;

type QuestId = String;

#[derive(Default, Encode, Decode, Debug, TypeInfo)]
pub struct Quests {
		/// Use admin until OpenGov is setup properly
		pub admin: ActorId,
		/// Mapping with base-tier quests: `QuestId` -> `BaseTierQuest`
		pub base_tier_quests: BTreeMap<QuestId, BaseTierQuest>,
		/// Mapping with mid-tier quests: `QuestId` -> `MidTierQuest`
		pub mid_tier_quests: BTreeMap<QuestId, MidTierQuest>,
		/// Mapping with top-tier quests: `QuestId` -> `TopTierQuest`
		pub top_tier_quests: BTreeMap<QuestId, TopTierQuest>,
		/// Track `QuestStatus` use `QuestId`
		pub quest_status: BTreeMap<QuestId, QuestStatus>,
		/// Listing of human and AI providers that are allowed to publish quests
		pub approved_providers: Vec<ActorId>,
}

static mut CONTRACT: Option<Quests> = None;

#[no_mangle]
extern "C" fn init() {
    let init_info: InitQuest = msg::load().expect("Failed to load init info");
    unsafe {
        CONTRACT = Some(Quests {
            admin: msg::source(),
            approved_providers: init_info.approved_providers,
            ..Default::default()
        });
    }

    let _ = msg::reply(QuestEvent::Ok { msg: String::from("Quest Contract Initiated!") }, 0);
}

#[no_mangle]
extern "C" fn handle() {
    let action: QuestAction = msg::load().expect("Failed to load action");
    let _quests: &mut Quests = unsafe { CONTRACT.as_mut().expect("Quest contract not initialized.") };

    match action {
        QuestAction::Publish => {
            let _ = msg::reply(QuestEvent::Ok { msg: String::from("Quest published!") }, 0);
        },
        QuestAction::Commit => {
            let _ = msg::reply(QuestEvent::Ok { msg: String::from("New commiter added!") }, 0);
        },
        QuestAction::Submit => {
            let _ = msg::reply(QuestEvent::Ok { msg: String::from("New submissions!") }, 0);
        },
        QuestAction::Grade => {
            let _ = msg::reply(QuestEvent::Ok { msg: String::from("Quest graded!") }, 0);
        },
        QuestAction::Modify => {
            let _ = msg::reply(QuestEvent::Ok { msg: String::from("Quest modified!") }, 0);
        },
        QuestAction::Extend => {
            let _ = msg::reply(QuestEvent::Ok { msg: String::from("Quest deadline extended!") }, 0);
        },
        QuestAction::Close => {
            let _ = msg::reply(QuestEvent::Ok { msg: String::from("Quest closed!") }, 0);
        },
        QuestAction::Retract => {
            let _ = msg::reply(QuestEvent::Ok { msg: String::from("Quest retracted!") }, 0);
        },
        QuestAction::Search => {
            let _ = msg::reply(QuestEvent::Ok { msg: String::from("Some actor searched a quest!") }, 0);
        },
    }
}