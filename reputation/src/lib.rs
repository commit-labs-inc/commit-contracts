#![no_std]
use gstd::{ msg, prelude::*, collections::BTreeMap };
use reputation_io::*;

/// The multi token implementation.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct MTC {
    tokens: BTreeMap<TokenId, Token>,
    owners: BTreeMap<Owner, GeneralOwnerData>,
}

static mut MTC: Option<MTC> = None;

#[no_mangle]
extern "C" fn init() {
    let init_info: InitMTK = msg::load().expect("Failed to load init info");

    let token = Token {
        name: init_info.name,
        total_supply: init_info.total_supply,
        owners: BTreeMap::new(),
    };

    let mut init_tokens_map = BTreeMap::new();
    init_tokens_map.insert(init_info.token_id, token);

    unsafe {
        MTC = Some(MTC {
            tokens: init_tokens_map,
            ..Default::default()
        });
    }

    let _ = msg::reply(String::from("Reputation contract initiated!"), 0);
}

#[no_mangle]
extern "C" fn handle() {
    let actions: MTKAction = msg::load().expect("Failed to load actions");
    let mtc: &mut MTC = unsafe { MTC.as_mut().expect("Multi-token contract not initialized!") };

    match actions {
        MTKAction::Mint { token_id, to, amount } => {
            // Check if the token with the id exists
            if !mtc.tokens.contains_key(&token_id) {
                let _ = msg::reply(MTKEvent::Err { msg: String::from("Token not exists!") }, 0);
            }
            // Check if the owner exists
            if !mtc.owners.contains_key(&to) {
                // If its a new user, add the entry
                mtc.owners.insert(to, GeneralOwnerData { balance: amount });
            } else {
                // If the user exists, update the balance
                let owner = mtc.owners.get_mut(&to).expect("Owner not found");
                owner.balance += amount;
            }

            let _ = msg::reply(MTKEvent::Ok { msg: String::from("Token minted!") }, 0);
        },
        MTKAction::GetBalance { token_id, owner } => {
            // Check if the token with the id exists
            if !mtc.tokens.contains_key(&token_id) {
                let _ = msg::reply(MTKEvent::Err { msg: String::from("Token not exists!") }, 0);
            }
            // Check if the owner exists
            if !mtc.owners.contains_key(&owner) {
                let _ = msg::reply(MTKEvent::Err { msg: String::from("Owner not exists!") }, 0);
            }

            let owner = mtc.owners.get(&owner).expect("Owner not found");
            let _ = msg::reply(MTKEvent::Ok { msg: format!("Balance: {}", owner.balance) }, 0);
        },
    }
}