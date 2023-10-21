#![no_std]
use gstd::{prelude::*, ActorId};
use gmeta::{In, InOut, Metadata, Out};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<AccountAction, AccountEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<State>;
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct State {
	pub owner_id: ActorId,
	pub max_num_accounts: u32,
	pub num_counter: u32,
	pub seekers: Vec<ActorId>,
	pub recruiters: Vec<ActorId>,
	pub accounts: Vec<(ActorId, Account)>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Account {
	pub username: String,
	// a two-value enum, seeker and recruiter
	pub role: Roles,
	// a collection of skill badge NFT ids
	pub badges: Vec<Badges>,
	// a collection of ids of quests claimed or published by the account
	// the quests are stored within quest contract for now,
	// but will be moved to de-centralized storage gradually
	pub quests: Vec<(String, Status)>,
	// this field is built by users to showcase the quests they've done
	// for seekers, it will be used for resume generation
	// for recruiters, it will be used for advertising
	pub quest_decks: Vec<(String, Status)>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Roles {
	Seeker,
	Recruiter,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Badges {
	// the human friendly name of a badge
	name: String,
	// represents the NFT id of a certain badge
	// the metainfos are stored within the NFT contract
	id: String,
	/// For seekers,
	/// this field represents how many times can a badge be used to purchase internship level quests
	/// For recruiters,
	/// this field represents the freshness or activeness of a recruiter,
	/// each time a recruiter issues the same badge, the number will go up and gradually go down with time.
	///
	/// Requirements:
	/// * this field MUST NOT exceeds 100 or below 0.
	amount: u8,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Status {
	Seeker(SeekerStatus),
	Recruiter(RecruiterStatus),
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum SeekerStatus {
	// status change after successfully claimed a quest
	Claimed,
	// status change after successfully submitted to a quest
	Submitted,
	
	// Below are unique statuses for Ads Bay.

	// status change after professor sent an interview invitation
	InterviewReceived,
    // status change after accepting an interview invitation
    InterviewAccepted,
	// status change after professor sent an offer
	OfferReceived,
    // status change after accepting an offer
    OfferAccepted,
	// status change after the formal enrollment completed
	Enrolled,
	// status change after professors clicking the "reject" button
	Rejected,

	// Below are unique statuses for Quest Harbor
	Accepted,
	GenerallyGood,
	NeedsImprovements,
	// status change after a quest is completed and minted by the NFT contract
	Minted,
	// represents no status found
	None,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RecruiterStatus {
	// status change after successfully published a quest
	Published,
	// status change after the deadline has passed & all gradeable submissions have been graded
	Completed,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum AccountAction {
	// TODO: add more changeable items.
	ChangeName {
		new_name: String,
	},
	Login {
		role: Roles,
	},
	/// Delete an account that associated with the sender's address.
	///
	/// Requirements:
	/// * only account owners can delete their own accounts
	Delete,

	/// Recruiter will send interview invitation to a seeker of a quest through quest contract.
    /// 
    /// Requirements:
    /// * only the quest contract and call this action
    /// 
    /// Arguments:
    /// * quest_id: the id of the quest that the recruiter wants to send interview invitation to a seeker
    /// * seeker_id: the id of the seeker that the recruiter wants to send interview invitation to
	SendInterview {
        quest_id: String,
        seeker_id: ActorId,
    },
	/// Recruiter will send offer proposal to a seeker of a quest through quest contract.
    /// 
    /// Requirements:
    /// * only the quest contract can call this action
    /// 
    /// Arguments:
    /// * quest_id: the id of the quest that the recruiter wants to send offer proposal to a seeker
	/// * recruiter_id: the id of the recruiter
    /// * seeker_id: the id of the seeker that the recruiter wants to send offer proposal to
    SendOffer {
        quest_id: String,
        recruiter_id: ActorId,
        seeker_id: ActorId,
    },
	/// Seeker will accept the interview invitation from a recruiter of a quest through quest contract.
	/// 
	/// Requirements:
	/// * only the quest contract can call this action
	/// 
	/// Arguments:
	/// * quest_id: the id of the quest that the seeker wants to accept interview invitation from a recruiter
	/// * seeker_id: the id of the seeker
	AcceptInterview {
		quest_id: String,
		seeker_id: ActorId,
	},
	/// Seeker will accept the offer proposal from a recruiter of a quest through quest contract.
	/// 
	/// Requirements:
	/// * only the quest contract can call this action
	/// 
	/// Arguments:
	/// * quest_id: the id of the quest that the seeker wants to accept offer proposal from a recruiter
	/// * seeker_id: the id of the seeker
	AcceptOffer {
		quest_id: String,
		seeker_id: ActorId,
	},
	/// Recruiter can reject after recieveing submission or after an interview through the quest contract.
	/// Only recruiter can manually reject a seeker for now.
	/// 
	/// Requirements:
	/// * only the quest contract can call this action
	/// 
	/// Arguments:
	/// * quest_id: the id of the quest that the recruiter wants to reject a seeker
	/// * seeker_id: the id of the seeker
	RecruiterReject {
		quest_id: String,
		seeker_id: ActorId,
	},
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum AccountEvent {
	ContractInitiated,
	MaxLimitReached {
		allowed_max: u32,
		current_num: u32,
	},
	AccountExists {
		username: String,
	},
	AccountCreated {
		account: ActorId,
		timestamp: u64,
	},
	NameChanged {
		account: ActorId,
		timestamp: u64,
	},
	AccountDeleted {
		account: ActorId,
		timestamp: u64,
	},
	InterviewReceived {
        quest_id: String,
        seeker_id: ActorId,
    },
    OfferReceived {
        quest_id: String,
        recruiter_id: ActorId,
        seeker_id: ActorId,
    },
	InterviewAccepted {
		quest_id: String,
		seeker_id: ActorId,
	},
	OfferAccepted {
		quest_id: String,
		seeker_id: ActorId,
	},
	Rejected {
		quest_id: String,
		seeker_id: ActorId,
	},
	Err {
		msg: String,
	}
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitAccount {
	pub max_num_accounts: u32,
}