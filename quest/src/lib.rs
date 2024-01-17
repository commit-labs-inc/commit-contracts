#![no_std]
use gstd::{collections::BTreeMap, exec, msg, prelude::*, ActorId};
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
    let quests: &mut Quests = unsafe { CONTRACT.as_mut().expect("Quest contract not initialized.") };

    match action {
        QuestAction::Publish { quest_type, quest_info } => {
            let _ = msg::reply(quests.publish(quest_type, quest_info), 0);
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

impl Quests {
    fn publish(&mut self, quest_type: QuestType, quest_info: IncomingQuest) -> QuestEvent {
        let quest_id = quest_id_gen();

        match quest_type {
            QuestType::BaseTier => {
                // 0. Only approved providers can publish quests
                if !self.is_approved(msg::source()) {
                    return QuestEvent::Err { msg: String::from("You are not an approved provider!") }
                }
                // 1. Construct the actual quest based on the incoming quest info
                let base_tier_quest = BaseTierQuest {
                    base: Base { 
                        provider: msg::source(),
                        institution_name: quest_info.institution_name,
                        quest_name: quest_info.quest_name,
                        description: quest_info.description,
                        deliverables: quest_info.deliverables,
                        deadline: quest_info.deadline,
                        capacity: quest_info.capacity,
                        skill_token_name: quest_info.skill_token_name,
                        open_try: quest_info.open_try,
                        contact_info: quest_info.contact_info,
                        ..Default::default()
                    },
                    // TODO: need to check against the minimum allowance
                    free_gradings: quest_info.free_gradings,
                };
                // 2. Insert the incoming quests into the quest mapping
                self.base_tier_quests.insert(quest_id.clone(), base_tier_quest);
                // 3. Insert the quest status into the quest status mapping
                self.quest_status.insert(quest_id.clone(), QuestStatus::Open);
                // 4. Return the event
                QuestEvent::Ok { msg: String::from("Base tier quest published!") }
            },
            _ => {
                QuestEvent::Ok { msg: String::from("Quest published!") }
            }
        }
    }

    /// Check against the approved providers list
    fn is_approved(&self, sender: ActorId) -> bool {
        self.approved_providers.contains(&sender)
    }
}

fn quest_id_gen() -> String {
    exec::block_timestamp().to_string()
}