#![no_std]
use gstd::prelude::*;
use codec::{Decode, Encode};
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
    pub accounts: HashMap<String, Profile>,
}

// TODO: add more fields in the future to support more features
pub struct Profile {
    pub username: String,
}

impl Accounts {
    pub fn login(&mut self, user_addr: String) -> AccountEvent {
        // 1. check if the actor_id is already registered
        if self.accounts.contains_key(&user_addr) {
            // 2. return login success if it is
            return AccountEvent::LoginSuccess;
        } else {
            // 3. otherwise, register the actor_id
            self.accounts.insert(user_addr, Profile {
                username: String::from("anonymous turtle"),
            });
            return AccountEvent::Registered;
        }
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum AccountAction {
    // login action will handle both register and login
    // String represents the address of a user
    Login(String),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum AccountEvent {
    LoginSuccess,
    LoginFailed,
    Registered,
}