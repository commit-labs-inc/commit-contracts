#![no_std]
use gstd::{prelude::*, ActorId};
use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata};
use scale_info::TypeInfo;
use hashbrown::HashMap;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    // the init logic will receive a JSON string from the factory contract contains the quest information
    type Init = In<String>;
    type Handle = InOut<QuestAction, QuestEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = String;
}

// possible actions for an individual quest
#[derive(Encode, Decode, TypeInfo)]
pub enum QuestAction {
    Claim,  // let user claim the quest
    Submit(String), // let user submit the quest
    Grade(ActorId, u8),  // let quest provider grade the quest
}

#[derive(Encode, Decode, TypeInfo)]
pub enum QuestEvent {
    Claimed,
    Submitted,
    Graded,
    // TODO: move this to a separate enum later
    ErrorClaimerExists,
    ErrorSubmitterNotExists,
    ErrorNotQuestOwner,
}

pub struct Quest {
    pub id: ActorId,                                // contract id of the quest
    pub owner: ActorId,                             // id of the quest provider
    pub name: String,
    pub description: String,
    pub deadline: u64,                              // gstd::exec::block_timestamp() 
    pub claimers: Vec<ActorId>,                     // list of claimers
    pub claimer_submit: HashMap<ActorId, String>,   // claimer id -> submitted results
    pub claimer_grade: HashMap<ActorId, u8>,        // use index of ActorId in claimers to index the grades, for now
}

impl Quest {
    // career aspirants cannot claim a quest twice
    pub fn claim(&mut self, claimer: ActorId) -> QuestEvent {
        if self.claimers.contains(&claimer) { return QuestEvent::ErrorClaimerExists;}
        self.claimers.push(claimer);
        self.claimer_submit.insert(claimer, String::from("No submission yet"));
        self.claimer_grade.insert(claimer, 0);

        return QuestEvent::Claimed;
    }

    // only existing claimers can submit to a quest
    pub fn submit(&mut self, claimer: ActorId, submit: String) -> QuestEvent {
        if !self.claimers.contains(&claimer) { return QuestEvent::ErrorSubmitterNotExists;}
        self.claimer_submit.insert(claimer, submit);

        return QuestEvent::Submitted;
    }

    // only quest provider can grade a quest
    // only existing claimers can be graded
    pub fn grade(&mut self, msg_sender: ActorId, recipient: ActorId, grade: u8) -> QuestEvent {
        if self.owner != msg_sender { return QuestEvent::ErrorNotQuestOwner;}
        if !self.claimers.contains(&recipient) { return QuestEvent::ErrorSubmitterNotExists;}
        self.claimer_grade.insert(recipient, grade);

        return QuestEvent::Graded;
    }
}

