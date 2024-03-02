#![no_std]
use gstd::prelude::*;
use quest_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = quest_io::State;

    // Return any quests that implemented the QuestTrait trait
    pub fn get_quest_by_id(state: State, quest_id: QuestId) -> MidTierQuest {
        for (id, quest) in state.mid_tier_quests {
            if *id == quest_id {
                return quest;
            }
        }

        panic!("Quest not found");
    }
    
}
