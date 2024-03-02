#![no_std]

use gstd::{collections::BTreeMap, exec, msg, prelude::*, ActorId};
use quest_io::*;
use quest_io::QuestId;

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
        /// Mapping with dedicated quests: `QuestId` -> `DedicatedQuest`
        pub dedicated_quests: BTreeMap<QuestId, DedicatedQuest>,
		/// Track `QuestStatus` use `QuestId`
		pub quest_status: BTreeMap<QuestId, QuestStatus>,
		/// Listing of human and AI providers that are allowed to publish quests
		pub approved_providers: Vec<ActorId>,
        /// For fast search of a quest without the need to loop through the keys of the quest mappings
        pub quests_to_tiers: BTreeMap<QuestId, QuestType>,
        pub minumum_free_gradings: u8,
}

static mut CONTRACT: Option<Quests> = None;

#[no_mangle]
extern "C" fn init() {
    let init_info: InitQuest = msg::load().expect("Failed to load init info");
    unsafe {
        CONTRACT = Some(Quests {
            admin: msg::source(),
            approved_providers: init_info.approved_providers,
            minumum_free_gradings: init_info.minumum_free_gradings,
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
        QuestAction::Commit { quest_id } => {
            let _ = msg::reply(quests.commit(&quest_id), 0);
        },
        QuestAction::Submit { quest_id, submission } => {
            let _ = msg::reply(quests.submit(&quest_id, submission), 0);
        },
        QuestAction::Grade { quest_id, commiter, submission, grading} => {
            let _ = msg::reply(quests.grade(&quest_id, commiter, submission, grading), 0);
        },
        QuestAction::Modify { quest_id, base_info } => {
            let _ = msg::reply(quests.modify(&quest_id, base_info), 0);
        },
        QuestAction::Close { quest_id } => {
            let _ = msg::reply(quests.close(&quest_id), 0);
        },
    }
}

#[no_mangle]
extern "C" fn state() {
    let contract = unsafe { CONTRACT.take().expect("Unexpected error in taking state") };
    msg::reply::<State>(contract.into(), 0).expect(
        "Failed to encode or reply with `<ContractMetadata as Metadata>::State` from `state()`",
    );
}

impl Quests {
    fn publish(&mut self, quest_type: QuestType, quest_info: IncomingQuest) -> QuestEvent {
        // Only approved providers can publish quests
        if !self.is_approved(msg::source()) {
            return QuestEvent::Err { msg: String::from("You are not an approved provider!") }
        }
        
        let quest_id = quest_id_gen();

        match quest_type {
            QuestType::BaseTier => {
                // 0. Sanity check first
                // Free gradings need to above a threshold
                if quest_info.free_gradings < self.minumum_free_gradings {
                    return QuestEvent::Err { msg: String::from("Free gradings need to be above the minimum threshold!") }
                }
                // 1. Construct the actual quest based on the incoming quest info
                let base_tier_quest = BaseTierQuest {
                    base: self.construct_quest_base(quest_info.clone()),
                    // TODO: need to check against the minimum allowance
                    free_gradings: quest_info.free_gradings,
                };
                // 2. Insert the incoming quests into the quest mapping
                self.base_tier_quests.insert(quest_id.clone(), base_tier_quest);
                // 3. Insert the quest status into the quest status mapping
                self.quest_status.insert(quest_id.clone(), QuestStatus::Open);
                // 4. Insert the quest id into the quest to tier mapping
                self.quests_to_tiers.insert(quest_id.clone(), QuestType::BaseTier);
                // 5. Return the event
                QuestEvent::Ok { msg: String::from(quest_id) }
            },
            QuestType::MidTier => {
                // 0. Sanity check first
                // Free gradings need to above a threshold
                if quest_info.free_gradings < self.minumum_free_gradings {
                    return QuestEvent::Err { msg: String::from("Free gradings need to be above the minimum threshold!") }
                }
                // 1. Construct the actual quest based on the incoming quest info
                let mid_tier_quest = MidTierQuest {
                    base: self.construct_quest_base(quest_info.clone()),
                    // TODO: need to check against the minimum allowance
                    free_gradings: quest_info.free_gradings,
                    hiring_for: quest_info.hiring_for,
                    skill_tags: quest_info.skill_tags,
                    reputation_nft: quest_info.reputation_nft,
                };
                // 2. Insert the incoming quests into the quest mapping
                self.mid_tier_quests.insert(quest_id.clone(), mid_tier_quest);
                // 3. Insert the quest status into the quest status mapping
                self.quest_status.insert(quest_id.clone(), QuestStatus::Open);
                // 4. Insert the quest id into the quest to tier mapping
                self.quests_to_tiers.insert(quest_id.clone(), QuestType::MidTier);
                // 5. Return the event
                QuestEvent::Ok { msg: String::from(quest_id) }
            },
            QuestType::TopTier => {
                // 0. Sanity check first
                // Application deadline must in the future
                if quest_info.application_deadline < exec::block_height() {
                    return QuestEvent::Err { msg: String::from("Application deadline needs to be in the future!") };
                }
                // 1. Construct the actual quest based on the incoming quest info
                let top_tier_quest = TopTierQuest {
                    base: self.construct_quest_base(quest_info.clone()),
                    prize: quest_info.prize,
                    application_deadline: quest_info.application_deadline,
                    reputation_nft: quest_info.reputation_nft,
                };
                // 2. Insert the incoming quests into the quest mapping
                self.top_tier_quests.insert(quest_id.clone(), top_tier_quest);
                // 3. Insert the quest status into the quest status mapping
                self.quest_status.insert(quest_id.clone(), QuestStatus::Open);
                // 4. Insert the quest id into the quest to tier mapping
                self.quests_to_tiers.insert(quest_id.clone(), QuestType::TopTier);
                // 5. Return the event
                QuestEvent::Ok { msg: String::from(quest_id) }
            },
            QuestType::Dedicated => {
                // 1. Construct the actual quest based on the incoming quest info
                let dedicated_quest = DedicatedQuest {
                    base: self.construct_quest_base(quest_info.clone()),
                    dedicated_to: quest_info.dedicated_to,
                };
                // 2. Insert the incoming quests into the quest mapping
                self.dedicated_quests.insert(quest_id.clone(), dedicated_quest);
                // 3. Insert the quest status into the quest status mapping
                self.quest_status.insert(quest_id.clone(), QuestStatus::Open);
                // 4. Insert the quest id into the quest to tier mapping
                self.quests_to_tiers.insert(quest_id.clone(), QuestType::Dedicated);
                // 5. Return the event
                QuestEvent::Ok { msg: String::from(quest_id) }
            }
        }
    }

    /// Opportunity seekers commit (claim) a quest
    fn commit(&mut self, quest_id: &QuestId) -> QuestEvent {
        // Everyone can commit to a quest, but that quest must exists
        if !self.quest_status.contains_key(quest_id) {
            return QuestEvent::Err { msg: String::from("Quest does not exist!") };
        }

        // 1. Check if the quest is open
        // Notice: quest is automatically closed after the deadline is passed, so we don't need to check that here
        if self.quest_status.get(quest_id).unwrap() != &QuestStatus::Open {
            return QuestEvent::Err { msg: String::from("Quest is not open!") };
        }
        
        let quest = self.get_quest(quest_id);

        // Notice that only mid-tier and top-tier capacity will change over each commit.
        // The initial value of other types of quest's capacity must set to > 1.
        if let Err(e) = quest.commit(msg::source()) {
            return QuestEvent::Err { msg: e };
        } else {
            if quest.get_capacity() == 0 {
                self.quest_status.insert(quest_id.clone(), QuestStatus::Full);
            }
            return QuestEvent::Ok { msg: String::from("Quest committed!") };
        }
    }

    /// The committer who committed to the quest can submit to the quest only once.
    /// There are not much to check for the submission action, since the check is done during the commit process.
    fn submit(&mut self, quest_id: &QuestId, submission: Submmision) -> QuestEvent {
        // The quest must exists
        if !self.quest_status.contains_key(quest_id) {
            return QuestEvent::Err { msg: String::from("Quest does not exist!") };
        }

        if self.quest_status.get(quest_id).unwrap() == &QuestStatus::Finished {
            return QuestEvent::Err { msg: String::from("You can't submit after a quest is finished!") };
        }

        // Find where the quest is in the quest mappings
        let quest = self.get_quest(quest_id);

        if let Err(e) = quest.submit(msg::source(), submission) {
            return QuestEvent::Err { msg: e };
        } else {
            return QuestEvent::Ok { msg: String::from("Submission successful!") };
        }
    }

    fn grade(&mut self, quest_id: &QuestId, commiter: ActorId, submission: Submmision, gradings: Gradings) -> QuestEvent {
        // The quest must exists
        if !self.quest_status.contains_key(quest_id) {
            return QuestEvent::Err { msg: String::from("Quest does not exist!") };
        }
        // Find where the quest_id is in the quest mappings
        let quest = self.get_quest(quest_id);

        if let Err(e) = quest.grade(msg::source(), commiter, submission, gradings) {
            return QuestEvent::Err { msg: e };
        } else {
            // If the capacity is 1, then that means the previous status if Full, so we change it to Open.
            if quest.get_capacity() == 1 {
                self.quest_status.insert(quest_id.clone(), QuestStatus::Open);
            }
            return QuestEvent::Ok { msg: String::from("Quest successfully graded!") };
        }
    }

    /// Each modification will send the whole quest information,
    /// since there are not efficient ways to know which part got modified and which part did not.
    /// 
    /// Currently, only base inforamtion are modifiable.
    fn modify(&mut self, quest_id: &QuestId, base_info: Modifiable) -> QuestEvent {
        let quest = self.get_quest(quest_id);
        

        if let Err(e) = quest.modify(msg::source(), base_info) {
            return QuestEvent::Err { msg: e };
        } else {
            return QuestEvent::Ok { msg: String::from("Quest modified!") };
        }
    }

    /// Close is designed to let providers close the quest before the deadline.
    /// After closing, a quest will be marked as Closed, and no more commits are allowed,
    /// but submissions and gradings are still allowed.
    fn close(&mut self, quest_id: &QuestId) -> QuestEvent {
        // The quest must exists
        if !self.quest_status.contains_key(quest_id) {
            return QuestEvent::Err { msg: String::from("Quest does not exist!") };
        }

        // Get the quest.
        let quest = self.get_quest(quest_id);

        // Only the owner of the quest can close.
        if msg::source() != quest.get_owner() {
            return QuestEvent::Err { msg: String::from("You are not the admin!") };
        }

        // Only can close if the quest is not in the status of Closed.
        if self.quest_status.get(quest_id).unwrap() == &QuestStatus::Closed {
            return QuestEvent::Err { msg: String::from("Quest is already closed!") };
        }

        // Close the quest.
        self.quest_status.insert(quest_id.clone(), QuestStatus::Closed);

        return QuestEvent::Ok { msg: String::from("Quest closed!") };
    }

    /// Construct the base of a quest
    fn construct_quest_base(&self, quest_info: IncomingQuest) -> Base {
        Base { 
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
        }
    }

    /// Check against the approved providers list
    fn is_approved(&self, sender: ActorId) -> bool {
        self.approved_providers.contains(&sender)
    }

    /// Return the quest to caller for further modification.
    fn get_quest(&mut self, quest_id: &QuestId) -> &mut dyn QuestTrait {
        let quest_type = self.quests_to_tiers.get(quest_id).unwrap();
        match quest_type {
            QuestType::BaseTier => {
                self.base_tier_quests.get_mut(quest_id).unwrap()
            },
            QuestType::MidTier => {
                self.mid_tier_quests.get_mut(quest_id).unwrap()
            },
            QuestType::TopTier => {
                self.top_tier_quests.get_mut(quest_id).unwrap()
            },
            QuestType::Dedicated => {
                self.dedicated_quests.get_mut(quest_id).unwrap()
            }
        }
    }
}

impl From<Quests> for State {
    fn from(quests: Quests) -> Self {
        let Quests {
            admin,
            base_tier_quests,
            mid_tier_quests,
            top_tier_quests,
            dedicated_quests,
            quest_status,
            approved_providers,
            quests_to_tiers,
            minumum_free_gradings,
        } = quests;

        let base_tier_quests = base_tier_quests
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        let mid_tier_quests = mid_tier_quests
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        let top_tier_quests = top_tier_quests
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        let dedicated_quests = dedicated_quests
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        let quest_status = quest_status
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        let quests_to_tiers = quests_to_tiers
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        Self {
            admin,
            base_tier_quests,
            mid_tier_quests,
            top_tier_quests,
            dedicated_quests,
            quest_status,
            approved_providers,
            quests_to_tiers,
            minumum_free_gradings,
        }
    }

}

/// Generate random id for quests
fn quest_id_gen() -> String {
    exec::block_timestamp().to_string()
}