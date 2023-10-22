#![no_std]
use gstd::{msg, prelude::*, debug, collections::BTreeMap, ActorId};
use quest_io::*;

struct Quests {
	// the publisher of the quest contract
	owner: ActorId,
	// the address of the account's master contract
	account_contract_addr: ActorId,
	// maximum number of quests a single quest storage contract should handle
	max_num_quests: u32,
	// current number of quests in this contract
	num_counter: u32,
	// String represents quest id
	quests: BTreeMap<String, Quest>,
}

static mut CONTRACT: Option<Quests> = None;

#[no_mangle]
extern "C" fn init() {
    let _init_info: String = msg::load().expect("Failed to load init info");

    let _ = msg::reply(QuestEvent::ContractInitiated, 0);
}

#[no_mangle]
extern "C" fn handle() {
    
}