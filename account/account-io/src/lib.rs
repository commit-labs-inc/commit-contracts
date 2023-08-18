#![no_std]
use gstd::{prelude::*, ActorId};
use parity_scale_codec::{Encode, Decode};
use gmeta::{In, InOut, Metadata};
use scale_info::TypeInfo;
use hashbrown::HashMap;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<AccountAction, AccountEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = String;
}

pub struct Accounts {
    pub seeker_accounts: HashMap<ActorId, SeekerProfile>,
    pub recruiter_accounts: HashMap<ActorId, RecruiterProfile>,
}

impl Accounts {
    // search for actor's profile, return error if not found
    pub fn search(&self, actor: ActorId) -> Result<Profile, &'static str> {
        // search in seeker_accounts
        if let Some(profile) = self.seeker_accounts.get(&actor) {
            return Ok(Profile::Seeker(profile.clone()));
        }
        // search in recruiter_accounts
        if let Some(profile) = self.recruiter_accounts.get(&actor) {
            return Ok(Profile::Recruiter(profile.clone()));
        }

        //TODO: probably need an error type
        Err("User not found!")
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Profile {
    Seeker(SeekerProfile),
    Recruiter(RecruiterProfile),
}

// TODO: add more fields in the future to support more features
#[derive(Encode, Decode, Clone, Default, TypeInfo)]
pub struct SeekerProfile {
    pub username: String,
    pub badges_received: Vec<Badge>,
    pub quests: Vec<(QuestId, Status)>,
}

impl SeekerProfile {
    pub fn update_status(&mut self, quest_id: &QuestId, new_status: Status) {
        // Iterate through the quests, finding the one with the matching QuestId
        for (qid, status) in &mut self.quests {
            if qid == quest_id {
                *status = new_status; // Update the status
                break;
            }
        }
    }

    pub fn claim(&mut self, quest_id: QuestId, status: Status) {
        self.quests.push((quest_id, status));
    }
}

#[derive(Encode, Decode, Clone, Default, TypeInfo)]
pub struct RecruiterProfile {
    pub company_name: String,
    pub quests_issued: Vec<(QuestId, Status)>,
    pub badges_received: Vec<Badge>,
    pub badges_issued: Vec<Badge>,
}

impl RecruiterProfile {
    pub fn update_status(&mut self, quest_id: &QuestId, new_status: Status) -> Result<(), &'static str> {
        // Iterate through the quests, finding the one with the matching QuestId
        for (qid, status) in &mut self.quests_issued {
            if qid == quest_id {
                *status = new_status; // Update the status
                return Ok(());
            }
        }

        // Return an error if the QuestId was not found
        Err("QuestId not found")
    }

    pub fn publish(&mut self, quest_id: QuestId, status: Status) {
        self.quests_issued.push((quest_id, status));
    }
}

#[derive(Encode, Decode, Clone, TypeInfo)]
pub enum Badge {
    DryLabResearcher,
}

#[derive(Encode, Decode, Clone, TypeInfo)]
pub enum Status {
    InProgress,
    Submitted,
    Accepted,
    GenerallyGood,
    NeedImprovements,
    Open,
    Full,
    Closed,
}

#[derive(Encode, Decode, Clone, TypeInfo, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QuestId(String);

impl QuestId {
    // TODO: need to add stricter rules for quest id
    pub fn new(id: String) -> Self {
        if id.len() != 20 {
            panic!("quest id must be 20 characters long");
        }
        Self(id)
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum AccountAction {
    // login action will handle both register and login
    Login {
        role: String, // seeker or recruiter
    },
    // when quest related happens, we need to record account info change through this action
    Record {
        subject: ActorId,
        action: String,
        quest_id: String,
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum AccountEvent {
    LoginSuccess {
        profile: Profile,
    },
    LoginFailed {
        subject: ActorId,
    },
    RecordSuccess,
    RecordFailed,
}