#![no_std]
use gstd::{prelude::*, ActorId, collections::BTreeMap};
use gmeta::{In, InOut, Metadata};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<QuestAction, QuestEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct Quest {
	// id of a quest, needs to change to a dedicated type in the future
	pub id: u32,
	// possible status of a quest
	pub status: QuestStatus,
	// recruiters who published the quest
	pub publisher: ActorId,
	// title of the quest, the type should be changed to a dedicated type for better security control
	pub title: String,
	// possible positions that a professor can offer
	pub position: Position,
	// deadline in the format of exec::block_height(),
	// after the deadline has passed, status will automatically change to closed,
	// no more claims and submissions allowed after the deadline. 
	pub deadline: u32,
	// a string of the uri of the image,
	// None means no image for this quest
	pub img: Option<String>,
	// descriptions of deliverables of this quest
	// this field is only displayed to seekers
	pub deliverables: Vec<String>,
	// String represents the uri of the actual submission,
	// for now we only accept one uri per seeker,
	// should use IPFS cid and accept more uris per seeker in the future.
	pub seeker_submission: BTreeMap<ActorId, String>,
	// seeker's status of a quest
	pub seeker_status: BTreeMap<ActorId, Status>,
	// this probably needs to change to a dedicated type for better screen display
	pub details: String,
}

impl Quest {
	// method to check if a quest is complete
	pub fn is_complete(&self) -> bool {
		!self.title.is_empty()
			&& !self.deliverables.is_empty()
			&& !self.details.is_empty()
	}
	// check if a quest is open
	pub fn is_open(&self) -> bool {
		self.status == QuestStatus::Open
	}
	// check if a quest is closed
	pub fn is_closed(&self) -> bool {
		self.status == QuestStatus::Closed
	}
	// check if a seeker's current status matches the status given
	pub fn seeker_status_match(&self, key: &ActorId, status: Status) -> bool {
		*self.seeker_status.get(key).unwrap() == status
	}
	// add a seeker's submission to the quest
	pub fn add_submission(&mut self, seeker: ActorId, submission: String) {
		self.seeker_submission.insert(seeker, submission);
	}
	// change a seeker's status of a quest
	pub fn change_seeker_status(&mut self, key: ActorId, status: Status) {
		self.seeker_status.insert(key, status);
	}
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum QuestStatus {
	Open,
	Full,
	Closed,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Position {
	Intern,
	Master,
	Doctor,
	PostDoc,
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum Status {
	Claimed,
	WaitingReply,
	InterviewReceived,
	InterviewAccepted,
	OfferReceived,
	OfferAccepted,
	Rejected,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct Ads {
	// channel of the ads, e.g. LinkedIn, WeChat and etc.
	channel: String,
	// link to the ads
	url: String,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum QuestAction {
	/// Recruiters publish a quest.
	/// 
	/// Requirements:
	/// * The msg sender must be an approved recruiter.
	/// * All fields of a quest must be filled.
	/// 
	/// Arguments:
	/// * quest: the quest to be published.
	Publish {
		quest: QuestInfo,
	},
	/// Seekers claim a quest.
	/// 
	/// Requirements:
	/// * the quest of the given id must exist.
	/// * the quest must be open.
	/// 
	/// Arguments:
	/// * quest_id: the id of the quest to be claimed.
	Claim {
		quest_id: u32,
	},
	/// Seekers submit their submissions.
	/// 
	/// Requirements:
	/// * the quest of the given id must exist.
	/// * the quest must NOT be closed.
	/// * the seeker must have claimed the quest.
	/// 
	/// Arguments:
	/// * quest_id: the id of the quest to be claimed.
	/// * submission: the submission of the seeker.
	Submit {
		quest_id: u32,
		// the external url of the submission, e.g. Google drive link
		// this should be changed to IPFS cid in the future
		submission: String,
	},
	/// Recruiters send interview invitations to seekers.
	/// 
	/// Requirements:
	/// * the quest of the given id must exist.
	/// * the seeker must have submitted to the quest.
	/// * the msg sender must be an approved recruiter.
	/// 
	/// Arguments:
	/// * seeker: the id of the seeker to be interviewed.
	/// * quest_id: the id of the quest.
	Interview {
		seeker: ActorId,
		quest_id: u32,
	},
	/// Seekers accept interview invitations.
	/// 
	/// Requirements:
	/// * the quest of the given id must exist.
	/// * the seeker must have received an interview invitation.
	/// 
	/// Arguments:
	/// * quest_id: the id of the quest.
	AcceptInterview {
		quest_id: u32,
	},
	/// Recruiters send offers to seekers.
	/// 
	/// Requirements:
	/// * the quest of the given id must exist.
	/// * the seeker must have accepted an interview invitation.
	/// * the msg sender must be an approved recruiter.
	/// 
	/// Arguments:
	/// * seeker: the id of the seeker to be offered.
	/// * quest_id: the id of the quest.
	Offer {
		seeker: ActorId,
		quest_id: u32,
	},
	/// Seekers accept offers.
	/// 
	/// Requirements:
	/// * the quest of the given id must exist.
	/// * the seeker must have received an offer.
	/// 
	/// Arguments:
	/// * quest_id: the id of the quest.
	AcceptOffer {
		quest_id: u32,
	},
	/// Recruiters reject seekers after they submitted or interviewed.
	/// 
	/// Requirements:
	/// * the quest of the given id must exist.
	/// * the seeker must have either submitted or interviewed.
	/// * the msg sender must be an approved recruiter.
	/// 
	/// Arguments:
	/// * seeker: the id of the seeker to be rejected.
	/// * quest_id: the id of the quest.
	Reject {
		seeker: ActorId,
		quest_id: u32,
	},
	// TODO: the logic of closing a quest needs to be discussed
	Close {
		quest_id: u32,
	},
	/// Add reruiters to the approved recruiters list.
	/// 
	/// Requirements:
	/// * The sender must be the owner of the quest contract.
	/// * The recruiter must not be in the approved recruiters list.
	/// 
	/// Arguments:
	/// * recruiter: the id of the recruiter to be added.
	AddRecruiter {
		recruiter: ActorId,
	},
	/// Change the stored address of the account contract.
	/// 
	/// Requirements:
	/// * The sender must be the owner of the quest contract.
	/// 
	/// Arguments:
	/// * new_account_contract: the new address of the account contract.
	ChangeAccountContract {
		new_account_contract: ActorId,
	},
}

#[derive(Encode, Decode, TypeInfo)]
pub struct QuestInfo {
    pub id: u32,
    pub title: String,
    pub position: Position,
    pub deadline: u32,
    pub img: Option<String>,
    pub deliverables: Vec<String>,
    pub details: String,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum QuestEvent {
	ContractInitiated,
	OperationSuccess {
		// name of the operation
		name: String,
		// exec::block_timestamp()
		timestamp: u64,
	},
	OperationErr {
		// name of the operation
		name: String,
		// why is it failed
		reason: String,
		// exec::block_timestamp()
		timestamp: u64,
	}
}