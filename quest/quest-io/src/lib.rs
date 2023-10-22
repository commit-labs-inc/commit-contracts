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
	id: String,
	// possible status of a quest
	status: QuestStatus,
	// recruiters who published the quest
	publisher: ActorId,
	// title of the quest, the type should be changed to a dedicated type for better security control
	title: String,
	// possible positions that a professor can offer
	position: Position,
	// deadline in the format of exec::block_height(),
	// after the deadline has passed, status will automatically change to closed,
	// no more claims and submissions allowed after the deadline. 
	deadline: u32,
	// a string of the uri of the image,
	// None means no image for this quest
	img: Option<String>,
	// descriptions of deliverables of this quest
	// this field is only displayed to seekers
	deliverables: Vec<String>,
	// String represents the uri of the actual submission,
	// for now we only accept one uri per seeker,
	// should use IPFS cid and accept more uris per seeker in the future.
	seeker_submission: BTreeMap<ActorId, String>,
	// seeker's status of a quest
	seeker_status: BTreeMap<ActorId, Status>,
	// urls to the published ads,
	// this field is only displayed to recruiters
	ads_links: Ads,
	// this probably needs to change to a dedicated type for better screen display
	details: String,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
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

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Status {
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
	Publish {
		publisher: ActorId,
		quest: Vec<u8>,
	},
	Claim {
		// the seeker's id who claimed the quest
		seeker: ActorId,
		// which quest is claimed
		quest_id: String,
	},
	Submit {
		// the seeker's id who submitted to the quest
		seeker: ActorId,
		quest_id: String,
		// the external url of the submission, e.g. Google drive link
		// this should be changed to IPFS cid in the future
		submission: String,
	},
	Interview {
		// the id of the seeker who will be interviewed
		seeker: ActorId,
		quest_id: String,
	},
	Offer {
		// the id of the seeker who is getting an offer
		seeker: ActorId,
		quest_id: String,
	},
	Reject {
		seeker: ActorId,
		quest_id: String,
	},
	Close {
		quest_id: String,
	}
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