#![no_std]

use gstd::{ collections::HashMap, prelude::*, ActorId };
use gmeta::{In, InOut, Out, Metadata};

pub struct ProgramMetadata;

pub type TokenId = u128;

impl Metadata for ProgramMetadata {
    type Init = In<InitMTK>;
    type Handle = InOut<MTKAction, Result<MTKEvent, MTKError>>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<State>;
}

#[derive(Debug, Default)]
pub struct MtkData {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub available_skill_names: AvailableSkillNames,
    // data fields for skill fungible tokens
    pub balances: HashMap<TokenId, HashMap<ActorId, u128>>,
    pub skill_fungible_tokens: HashMap<TokenId, SkillFtData>,
    pub ft_owners: HashMap<ActorId, Vec<TokenId>>,
    // for skill nfts
    pub skill_nft_metadata: HashMap<TokenId, SkillNftMetadata>,
    pub nft_owners: HashMap<ActorId, Vec<TokenId>>,
}

// For skill fungible tokens.
#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone, PartialEq, Eq)]
pub struct SkillFtData {
    // e.g. Biomedicine
    pub name: Option<String>,
	// Location of the symbol
	pub symbol: Option<String>,
	// Total circulation of the token
	pub circulation: Option<u128>,
}

#[derive(Debug, Decode, Encode, TypeInfo, Default)]
pub struct SkillNftMetadata {
	// The receipient's id
	pub owner: Option<ActorId>,
	// The name of the skill NFT
	pub title: Option<String>,
	// This should be an overview of the quest that issued this NFT
	pub description: Option<String>,
	// All the information of the quest, including the submission from the NFT receipient.
	pub quest_details: Option<Quest>,
	// Freshness score denominated in block height
	pub freshness: Option<u32>,
}

#[derive(Debug, Decode, Encode, TypeInfo, Default)]
// All possible names of skill NFTs we can issue.
pub struct AvailableSkillNames {
    names: Vec<String>,
}

impl AvailableSkillNames {
    pub fn new() -> Self {
        AvailableSkillNames {
            names: Vec::new(),
        }
    }

    pub fn add_name(&mut self, name: String) {
        self.names.push(name);
    }

    pub fn remove_name(&mut self, name: &String) -> bool {
        if !self.check_name(name) {
            // return false if the name does not exists
            return false;
        }

        self.names.retain(|n| n != name);
        true
    }

    pub fn check_name(&self, name: &String) -> bool {
        self.names.contains(name)
    }
}

#[derive(Encode, Decode, Debug, TypeInfo, Default)]
pub struct Quest {
    // The quest's id
    pub id: Option<String>,
    // The quest's publisher
    pub publisher: Option<ActorId>,
    // The quest's title
    pub title: Option<String>,
    // The quest's description
    pub description: Option<String>,
    // The quest's reward
    pub reward: Option<TokenId>,
    // The submission storage id `String` that resulted in the issuance of the skill NFT.
    pub submission: Option<String>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitMTK {
    /// Multitoken name.
    pub name: String,
    /// Multitoken symbol.
    pub symbol: String,
    /// Multitoken base URI.
    pub base_uri: String,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MTKAction {
    /// Mint a skill ft to a user.
    ///
    /// # Requirements:
    /// * only existing tokens can be minted.
    ///
    /// On success returns `MTKEvent::SkillTokenMinted`.
    MintFtTo {
        /// Token id.
        id: TokenId,
        /// Token amount.
        amount: u128,
        /// Receipient.
        to: ActorId,
    },
    // Burn skill fts to gain entrance to mid-tier quests.
    //
    // # Requirements:
    // * the account `from` must holds enough `amount` fts
    //
    // On success returns `MTKEvent::SkillTokenBurned`.
    Burn {
        // The skill ft's id.
        id: TokenId,
        // The account to burn.
        from: ActorId,
        // The amount of skill fts to burn.
        amount: u128,
    },
    /// Mint a new skill nft to receipient.
    ///
    /// On success returns `MTKEvent::SkillNftMinted`.
    MintNftTo {
        // The receipient.
        to: ActorId,
        // Metadata for the skill nft.
        metadata: SkillNftMetadata,
    },

    /// Verify a user's reputation using its on-chain address.
    ///
    /// # Requirements:
    /// * the id to be verified must exists.
    ///
    /// On success returns `MTKEvent::RepuVerified`.
    VerifyReputation {
        // Which user to verify
        target: ActorId,
        // What reputation type to verify: skill ft or nft
        // `true` - nft, `false` - ft
        skill_type: bool,
        // what reputation to verify
        token_id: TokenId,
    },
    

    // -----------------Below are functions that must go through OpenGov in the future-----------------
    
    /// Add a new skill FT.
    /// 
    /// # Requirements:
    /// * only contract creator can add new skill fts.
    /// 
    /// On success returns `MTKEvent::NewFtAdded`.
    AddFt {
        // The metadata of the newly added skill token.
        token_data: SkillFtData,
    },

    /// Change the information of a skill FT.
    ///
    /// # Requirements:
    /// * token id must exists.
    /// * only contract creater can change the information.
    ///
    /// On success returns `MTKEvent::SkillFtChanged`.
    ChangeFt {
        // Which token to change.
        id: TokenId,
        // The metadata after changing.
        new_data: SkillFtData,
    },

    /// Add a new possible name for skill NFTs.
    ///
    /// # Requirements:
    /// * only contract creator can add new skill NFT.
    ///
    /// On success returns `MTKEvent::SkillNftAdded`.
    AddNft {
        // The new name.
        name: String,
    },

    /// Render an NFT not issuable anymore.
    /// Notice: it simply removes the name from the available names set so it can't get issued in the future.
    ///
    /// # Requirements:
    /// * the name to be removed must exists.
    /// * only contract creator can remove a skill NFT.
    ///
    /// On success returns `MTKEvent::SkillNftRemoved`.
    RemoveNft {
        // Which NFT name to remove.
        name: String,
    },

    
    /// Change the base URI of the multitoken.
    /// 
    /// # Requirements:
    /// * only contract creator can change the base URI.
    ///
    /// On success returns `MTKEvent::MtkUriChanged`.
    ChangeBaseUri {
        new_base_uri: String,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MTKEvent {
    SkillTokenMinted {
        id: TokenId,
        amount: u128,
        to: ActorId,
    },
    SkillTokenBurned {
        from: ActorId,
        id: TokenId,
        amount: u128,
    },
    SkillNftMinted {
        id: TokenId,
        to: ActorId,
    },
    RepuVerified {
        // The address which initiated the verification.
        initiator: ActorId,
        // The target address that get verified.
        target: ActorId,
    },
    RepuVerificationFail {
        // The address which initiated the verification.
        initiator: ActorId,
        // The target address that get verified.
        target: ActorId,
    },
    NewFtAdded {
        id: TokenId,
    },
    SkillFtChanged {
        id: TokenId,
    },
    SkillNftAdded {
        name: String,
    },
    SkillNftRemoved {
        name: String,
    },
    MtkUriChanged {
        new_uri: String,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MTKError {
    TokenDoesNotExists,
    TokenAlreadyExists,
    OnlyCreaterCanOperate,
    InsufficientBalance,
    OwnerDoesNotExists,
    SkillNameDoesNotExists,
    NotImplemented,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct State {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub creator: ActorId,
    pub available_skill_names: AvailableSkillNames,
    pub balances: Vec<(TokenId, Vec<(ActorId, u128)>)>,
    pub skill_fungible_tokens: Vec<(TokenId, SkillFtData)>,
    pub ft_owners: Vec<(ActorId, Vec<TokenId>)>,
    pub skill_nft_metadata: Vec<(TokenId, SkillNftMetadata)>,
    pub nft_owners: Vec<(ActorId, Vec<TokenId>)>,
}