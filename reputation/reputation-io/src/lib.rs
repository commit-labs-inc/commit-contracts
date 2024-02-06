#![no_std]
use gstd::{ prelude::*, ActorId, collections::HashMap };
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
    pub balances: HashMap<TokenId, HashMap<ActorId, u128>>,
    pub matching_pairs: HashMap<TokenId, TokenId>,
    pub fungible_tokens: Vec<TokenId>,
    pub skill_nfts: Vec<TokenId>,
    pub repu_nfts: Vec<TokenId>,
    // for nft
    pub token_metadata: HashMap<TokenId, TokenMetadata>,
    pub owners: HashMap<TokenId, ActorId>,
}

#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone, PartialEq, Eq)]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub reference: Option<String>,
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
    /// Mints a skill ft or reputation nft to a user.
    ///
    /// # Requirements:
    /// * if minting a reputation NFT `amount` MUST equal to 1.
    /// * only existing tokens can be minted.
    ///
    /// On success returns `MTKEvent::MintTo`.
    Mint {
        /// Token id.
        id: TokenId,
        /// Token amount.
        amount: u128,
        /// Receipient.
        to: ActorId,
        /// Token metadata, applicable if minting an NFT.
        token_metadata: Option<TokenMetadata>,
    },
    // Transform skill ft to matching nft.
    //
    // # Requirements:
    // * at the fresh exchange rate, seeker must holds enough fts for `amount` of nfts.
    // * the `skill_ft` and `skill_nft` must in the matching pair.
    //
    // On success returns `MTKEvent::SkillNftAcquired`.
    /*
    BurnToNFT {
        /// Seeker.
        from: ActorId,
        /// The skill ft intended to spend.
        skill_ft: TokenId,
        /// The skill nft intended to acquire.
        skill_nft: TokenId,
        /// The amount of skill nfts intended to acquire.
        amount: u128,
    }
    */
    // -----------------Below are admin functions-----------------
    // Add a new token to the multitoken.
    AddSkillFt {
        // Token id.
        ft_id: TokenId,
        // The matching skill nft's id.
        nft_id: TokenId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MTKEvent {
    MintTo {
        to: ActorId,
        id: TokenId,
        amounts: u128,
    },
    SkillNftAcquired {
        receipient: ActorId,
        skill_nft: TokenId,
        amount: u128,
    },
    NewFtAdded {
        ft_id: TokenId,
        nft_id: TokenId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MTKError {
    TokenDoesNotExists,
    TokenAlreadyExists,
    NotImplemented,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct State {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub balances: Vec<(TokenId, Vec<(ActorId, u128)>)>,
    // matching pairs for skill ft and nft
    pub matching_pairs: Vec<(TokenId, TokenId)>,
    pub ft_tokens: Vec<TokenId>,
    pub skill_nft_tokens: Vec<TokenId>,
    pub repu_nft_tokens: Vec<TokenId>,
    pub token_metadata: Vec<(TokenId, TokenMetadata)>,
    // owner for nft
    pub owners: Vec<(TokenId, ActorId)>,
    pub creator: ActorId,
    pub supply: Vec<(TokenId, u128)>,
}

impl State {
    pub fn tokens_ids_for_owner(&self, owner: &ActorId) -> Vec<TokenId> {
        let mut tokens: Vec<TokenId> = Vec::new();
        let balances = &self.balances;
        for (token, bals) in balances {
            if bals.iter().any(|(id, _b)| owner.eq(id)) {
                tokens.push(*token);
            }
        }
        tokens
    }
    pub fn get_balance(&self, account: &ActorId, id: &TokenId) -> u128 {
        if let Some((_token_id, balances)) = self
            .balances
            .iter()
            .find(|(token_id, _balances)| id.eq(token_id))
        {
            if let Some((_owner, balance)) =
                balances.iter().find(|(owner, _balance)| account.eq(owner))
            {
                return *balance;
            }
        }
        0
    }
}