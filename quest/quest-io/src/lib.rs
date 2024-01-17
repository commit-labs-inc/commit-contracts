#![no_std]
use gstd::{prelude::*, ActorId, collections::BTreeMap};
use gmeta::{In, InOut, Metadata};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<InitQuest>;
    type Handle = InOut<QuestAction, QuestEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}

/// Init the quest contract with a list of approved providers
#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitQuest {
	pub approved_providers: Vec<ActorId>,
	pub minumum_free_gradings: u8,
}

/// Base structure for all quests
#[derive(Default, Debug, Encode, Decode, TypeInfo)]
pub struct Base {
	/// Security requirements:
	/// 1. institution and quest name needs to conform to social norm.
	pub institution_name: String,
	pub quest_name: String,
	/// Short descriptions about quests, ideally <= 10 sentences.
	pub description: String,
	/// Describe what are expected as submissions from seekers.
	pub deliverables: String,
	/// Specify the maximum seekers this quest willing to accept.
	/// That means it can handle at most $capacity concurrent ongoing seekers.
	pub capacity: u32,
	/// Specify which token (singular) will be issued as rewards.
	pub skill_token_name: SkillToken,
	/// Specify deadline in the format of Vara block height.
	///
	/// Functional requirements:
	/// 1. the specified deadline must > current block height.
	pub deadline: u64,
	/// Specify whether the quest will consume seekers free trying numbers or not.
	/// 
	/// Recommendations:
	/// 1. Top-tier quests are always set to 'True' - not consume.
	/// 2. Mid-tier quests are encouraged to set to 'False' - consume to let seekers jump-start with cautious.
	/// 3. Base-tier quests are encouraged to set to 'True' to let seekers bootstrap themselves.
	pub open_try: bool,
	/// The wallet address used to publish the quest.
	pub provider: ActorId,
	/// The person who is directly in charge of managing this quest.
	pub provider_name: String,
	/// Link to other social apps, e.g. gmail, X, and etc.
	///
	/// Security requirements:
	/// 1. validity of the links should be checked by us after publishing.
	/// 2. malformed links should be checked automatically before publishing.
	/// 3. notifications should be displayed to user about potential security issues.
	pub contact_info: String,
	// ----------------------------------------------------------------------------
	// Below are dynamic informations for a quest

	/// Manage submissions from seekers.
	/// We use google drive links now and will transition to include decentralized storage in the future.
	pub submissions: BTreeMap<ActorId, SeekerStatus>,
	/// Manage gradings for seekers.
	pub gradings: BTreeMap<ActorId, Option<Gradings>>,
	/// A quest can only get extended beyond its deadline once.
	pub extended: bool,
	/// A quest can only get modified once within a time limit start from the appearace of the first claimer.
	pub modified: bool,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct IncomingQuest {
	pub institution_name: String,
	pub quest_name: String,
	pub description: String,
	pub deliverables: String,
	pub capacity: u32,
	pub skill_token_name: SkillToken,
	pub deadline: u64,
	pub open_try: bool,
	pub provider_name: String,
	pub contact_info: String,
	pub free_gradings: u8,
	pub hiring_for: String,
	pub skill_tags: SkillNFT,
	pub reputation_nft: RepuNFT,
	pub prize: String,
	pub application_deadline: u32,
	pub dedicated_to: Option<Vec<ActorId>>,
}

// Base Tier - Skill Assessment Quest
#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct BaseTierQuest {
	pub base: Base,
	/// Specified by the quest providers, how many free gradings they are willing to hand out to seekers.
	/// Seekers who submitted without any free gradings left will be charged a minor amount of fee that will splitted between providers and Commit platform.
	///
	/// Functional requirements:
	/// 1. range needs to > MIN_LIMIT.
	pub free_gradings: u8,
}

// Mid Tier - Hiring Purpose Quest
#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct MidTierQuest {
	pub base: Base,
	/// Specified by the quest providers, how many free gradings they are willing to hand out to seekers.
	/// Seekers who submitted without any free gradings left will be charged a minor amount of fee that will splitted between providers and Commit platform.
	///
	/// Functional requirements:
	/// 1. range needs to > MIN_LIMIT.
	pub free_gradings: u8,
	/// Specify the position the provider is hiring for, e.g. Master, Ph.D., Internship.
	pub hiring_for: String,
	/// Specify which type of skill NFT is needed to start working on this quest.
	pub skill_tags: SkillNFT,
	/// Specify which reputation will be issued as rewards.
	/// Notice that there is also an implicit reward - internship opportunity.
	pub reputation_nft: RepuNFT,
}

// Top Tier - Competition Quest
#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct TopTierQuest {
	pub base: Base,
	/// Specified by the competition organizer.
	///
	/// Security requirements:
	/// 1. disclaimers need to display to seekers.
	pub prize: String,
	/// Specify deadline in the format of Vara block height.
	/// This deadline is different than the `deadline` field in the base structure,
	/// here it means after which users are not able to claim this quest anymore.
	///
	/// Functional requirements:
	/// 1. the specified deadline must > the current Vara block height.
	pub application_deadline: u32,
	/// Specify which reputation will be issued as rewards.
	/// Notice that there is also an implicit reward - global recognition (fame)
	pub reputation_nft: RepuNFT,
}

// Dedicated Quest
#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct DedicatedQuest {
	pub base: Base,
	/// Specify the wallet addresses that can claim this quest.
	///
	/// If this part is left empty, it means anyone knows the passcodes can claim this quest,
	/// suitable for the usage by providers who can't gather all the wallet addressess needed upfront, e.g. online courses and etc.
	pub dedicated_to: Option<Vec<ActorId>>,
}

/// The status of a seeker for a quest.
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum SeekerStatus {
	Waiting,
	Submitted(String),
	Graded(Gradings),
}

/// Possible gradings for every quest.
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Gradings {
	Accept,
	Good,
	Reject,
}

/// List all possible skill tokens we support.
/// This list should be manageable through OpenGov.
#[derive(Default, Debug, Encode, Decode, TypeInfo, Clone)]
pub enum SkillToken {
	#[default]
	None,
	Python,
	Simulation,
}

/// List all possible skill badges we can issue, they should be matched 1-1 to skill tokens.
/// This list should be manageable through OpenGov.
#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub enum SkillNFT {
	Python,
	Simulation,
}

/// List all possible reputation nfts we can provide, this should be more generall then the skill nfts.
/// This list should be manageable through OpenGov, but preferabily with a faster voting process setup.
#[derive(Default, Debug, Encode, Decode, TypeInfo, Clone)]
pub enum RepuNFT {
	#[default]
	None,
	CSHackathonWinner,
	ResearchCompetitionWinner,
	CSInternship,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum QuestStatus {
	Open,
	Full,
	Closed,
}

/// All possible quest types supported for now.
#[derive(Encode, Decode, TypeInfo)]
pub enum QuestType {
	BaseTier,
	MidTier,
	TopTier,
	Dedicated,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum QuestAction {
	/// Providers publish a quest.
	/// 
	/// Requirements:
	/// * The msg sender must be an approved recruiter.
	/// * All fields of a quest must be filled.
	/// 
	/// Arguments:
	/// * quest_type: the type of the quest to be published.
	Publish {
		quest_type: QuestType,
		quest_info: IncomingQuest,
	},
	/// Seekers claim a quest.
	/// 
	/// Requirements:
	/// * the quest of the given id must exist.
	/// * the quest must be open.
	/// 
	/// Arguments:
	/// * quest_id: the id of the quest to be claimed.
	Commit,
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
	Submit,
	
	Grade,
	Close,
	Extend,
	Modify,
	Retract,
	Search,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum QuestEvent {
	Ok {
		msg: String,
	},
	Err {
		msg: String,
	},
}