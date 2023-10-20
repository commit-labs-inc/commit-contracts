#![no_std]
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use quest_io::*;
use account_io::QuestId;

#[metawasm]
pub mod metafns {

    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn get_actor_quests(state: State, actor: ActorId, role: u8) -> Vec<(QuestId, Quest)> {
        match role {
            0 => {
                let mut quests = Vec::new();
                for quest in state.claimers_quests.iter() {
                    if quest.0 == &actor {
                        for quest_id in quest.1.iter() {
                            quests.push((quest_id.clone(), state.quests.get(quest_id).unwrap().clone()));
                        }

                        return quests;
                    }
                }

                Vec::new()
            },
            1 => {
                let mut quests = Vec::new();
                for quest in state.quests.iter() {
                    if quest.1.owner == actor {
                        quests.push((quest.0.clone(), quest.1.clone()));
                    }
                }
                quests
            },
            _ => Vec::new(),
        }
    }

    pub fn get_all_quests(state: State) -> Vec<(QuestId, Quest)> {
        let mut quests = Vec::new();
        for quest in state.quests.iter() {
            quests.push((quest.0.clone(), quest.1.clone()));
        }
        quests
    }

}