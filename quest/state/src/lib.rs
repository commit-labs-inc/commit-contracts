#![no_std]
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use quest_io::*;

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn seeker_state(state: State, actor: ActorId, role: u8) -> Vec<Quest> {
        match role {
            0 => {
                let mut quests = Vec::new();
                for quest in state.claimers_quests.iter() {
                    if quest.0 == &actor {
                        for quest_id in quest.1.iter() {
                            quests.push(state.quests.get(quest_id).unwrap().clone());
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
                        quests.push(quest.1.clone());
                    }
                }
                quests
            },
            _ => Vec::new(),
        }
    }
}