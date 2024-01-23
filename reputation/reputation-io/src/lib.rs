#![no_std]
use gstd::{ prelude::*, collections::BTreeMap, ActorId };
use gmeta::{In, InOut, Metadata};
use primitive_types::U256;

pub struct ProgramMetadata;

pub type Amount = U256;
pub type Owner = ActorId;
pub type TokenId = String;

impl Metadata for ProgramMetadata {
    type Init = In<InitMTK>;
    type Handle = InOut<MTKAction, MTKEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitMTK {
    pub token_id: TokenId,
    pub name: String,
    pub total_supply: Amount,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MTKAction {
    Mint {
        token_id: TokenId,
        to: Owner,
        amount: Amount,
    },
    GetBalance {
        token_id: TokenId,
        owner: Owner,
    }
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MTKEvent {
    Ok {
        msg: String,
    },
    Err {
        msg: String,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq)]
pub struct GeneralOwnerData {
    pub balance: Amount,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq)]
pub struct Token {
    pub name: String,
    pub total_supply: Amount,
    pub owners: BTreeMap<Owner, GeneralOwnerData>,
}