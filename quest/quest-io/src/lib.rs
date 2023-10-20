#![no_std]
use gstd::{prelude::*, ActorId};
use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata};
use account_io::{QuestId, Badge};
use scale_info::TypeInfo;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<QuestAction, QuestEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Quests;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum QuestAction {
    Claim {
        quest_id: QuestId,
    },
    Submit {
        quest_id: QuestId,
        submission: Submission,
    },
    Publish {
        quest: Quest,
    },               
    Grade {
        qeust_id: QuestId,
        claimer: ActorId,
        grades: Grading,
    },
}

#[derive(Encode, Decode, TypeInfo)]
pub enum QuestEvent {
    QuestPublished {
        recruiter: ActorId,
        quest_id: QuestId,
    },
    QuestClaimed {
        seeker: ActorId,
        quest_id: QuestId,
    },
    SubmissionReceived {
        seeker: ActorId,
        quest_id: QuestId,
    },
    QuestGraded {
        quest_id: QuestId,
        seeker: ActorId,
    },
    
    PublishError {
        recruiter: ActorId,
        time: u32, // gstd::exec::block_height()
    },
    ClaimError {
        seeker: ActorId,
        quest_id: QuestId,
        time: u32, // gstd::exec::block_height()
    },
    SubmitError {
        seeker: ActorId,
        quest_id: QuestId,
        time: u32, // gstd::exec::block_height()
    },
    GradeError {
        recruiter: ActorId,
        quest_id: QuestId,
        time: u32, // gstd::exec::block_height()
    },
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Submission {
    pub external_link: String,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub enum Grading {
    Accepted,
    GenerallyGood,
    NeedImprovements,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Quest {                       
    pub owner: ActorId,                             // id of the quest provider
    pub entity_name: Name,                        // name of the entity that provides the quest
    pub location: Location,                            // location of the entity
    pub communication_language: Vec<Language>,        // list of languages the entity can communicate in
    // TODO: need to change this into supporting multiple channels in the future
    pub communication_channel: Vec<Channel>,              // email that the entity can be reached at
    pub occupation: Vec<Occupation>,                        // list of occupations that the entity is looking for
    pub quest_name: Name,                         
    pub description: Description,
    // TODO: need to provide NFT badges in the future
    pub skill_badges: Vec<Badge>,                 // list of skill badges that will be provided upon completion
    pub max_claimers: ClaimerNum,                            // max number of claimers, 0 indicates no limit
    pub who_can_claim: ActorId,                   // id of the actor that can claim the quest
    pub deadline: Deadline,                              // gstd::exec::block_timestamp() 
    pub claimers: Vec<ActorId>,                    // list of claimers
    pub claimer_submit: Vec<(ActorId, Submission)>,   // claimer id -> submitted results
    pub claimer_grade: Vec<(ActorId, Grading)>,        // use index of ActorId in claimers to index the grades, for now 
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub struct Name(String);

impl Name {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub enum Location {
    NorthAmerica,
    Asia,
    Europe,
    Antarctica,
    Africa,
    SouthAmerica,
    Australia,
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub enum Occupation {
    Scientist,
    SoftwareEngineer,
    Designer,
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub enum Language {
    English,
    MandarinChinese,
    Hindi,
    Japanese,
    Spanish,
    French,
    Arabic,
    Russian,
    German,
    Turkish,
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub enum Channel {
    Email {
        address: String,
    },
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub struct Description(String);

impl Description {
    pub fn new(description: String) -> Self {
        if description.len() > 1000 {
            panic!("Description cannot be longer than 1000 characters");
        }
        Self(description)
    }
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub struct ClaimerNum(u8);

impl ClaimerNum {
    pub fn new(num: u8) -> Self {
        if num > 100 {
            panic!("Max number of concurrent claimers cannot be larger than 100");
        }
        Self(num)
    }
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub struct Deadline(u32);

impl Deadline {
    pub fn new(deadline: u32) -> Self {
        if deadline < gstd::exec::block_height() {
            panic!("Deadline cannot be earlier than current timestamp");
        }
        Self(deadline)
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Quests {
    // String is the id of the quest
    // TODO: need to change String into a dedicated type
    pub quests: BTreeMap<QuestId, Quest>,
    pub claimers_quests: BTreeMap<ActorId, Vec<QuestId>>,
}

impl Quests {
    // TODO: EVERY function needs a much stricter access control!

    pub fn publish(&mut self, quest: Quest, quest_id: QuestId) -> QuestEvent {
        if self.quests.contains_key(&quest_id) {
            return QuestEvent::PublishError {
                recruiter: quest.owner,
                time: gstd::exec::block_height(),
            }
        }

        let owner = quest.owner;
        self.quests.insert(quest_id.clone(), quest);

        QuestEvent::QuestPublished {
            recruiter: owner,
            quest_id,
        }
    }

    pub fn claim(&mut self, seeker: ActorId, quest_id: QuestId) -> QuestEvent {
        if !self.quests.contains_key(&quest_id) {
            return QuestEvent::ClaimError {
                seeker,
                quest_id,
                time: gstd::exec::block_height(),
            }
        }
        // 1. get the corresponding quest
        let quest = self.quests.get_mut(&quest_id).unwrap();
        // 2. add claimer to the quest's claimers list
        quest.claimers.push(seeker.clone());
        // 3. add claimer to the quest's claimer_quest mapping list
        self.claimers_quests.entry(seeker).or_insert(Vec::new()).push(quest_id.clone());
        QuestEvent::QuestClaimed {
            seeker,
            quest_id,
        }
    }

    pub fn submit(&mut self, seeker: ActorId, quest_id: QuestId, submission: Submission) -> QuestEvent {
        if !self.quests.contains_key(&quest_id) {
            return QuestEvent::SubmitError {
                seeker,
                quest_id,
                time: gstd::exec::block_height(),
            }
        }
        // 1. get the corresponding quest
        let quest = self.quests.get_mut(&quest_id).unwrap();
        // 2. add submission to the quest's claimer_submit mapping list
        quest.claimer_submit.push((seeker.clone(), submission));
        QuestEvent::SubmissionReceived {
            seeker,
            quest_id,
        }
    }

    pub fn grade(&mut self, seeker: ActorId, quest_id: QuestId, grades: Grading) -> QuestEvent {
        if !self.quests.contains_key(&quest_id) {
            return QuestEvent::GradeError {
                recruiter: seeker,
                quest_id,
                time: gstd::exec::block_height(),
            }
        }
        // 1. get the corresponding quest
        let quest = self.quests.get_mut(&quest_id).unwrap();
        // 2. add grading to the quest's claimer_grade mapping list
        quest.claimer_grade.push((seeker.clone(), grades));
        QuestEvent::QuestGraded {
            quest_id,
            seeker,
        }
    }
}