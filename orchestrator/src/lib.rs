#![no_std]
use gstd::{msg, prelude::*, debug, ActorId};
use orchestrator_io::{OrchestratorEvent, OrchestratorAction, Orchestrator};

static mut ORCHESTRATOR: Option<Orchestrator> = None;

#[no_mangle]
extern "C" fn init() {
    let blank: String = msg::load().expect("Fail to load message");
    debug!("Blank: {}", blank);
    unsafe {
        ORCHESTRATOR = Some(Orchestrator {
            routes: HashMap::new(),
        });

    }

    init_routes();
}

#[no_mangle]
extern "C" fn handle() {
    let action: OrchestratorAction = msg::load().expect("Fail to load message");
    let orchestrator = unsafe { ORCHESTRATOR.as_mut().unwrap() };
    match action {
        OrchestratorAction::Route(action_name, actor) => {
            debug!("Routing action: {}", action_name.clone());
            match orchestrator.route(action_name, actor) {
                OrchestratorEvent::Routed(action_name) => {
                    debug!("Routed action: {}", action_name);
                    msg::reply(OrchestratorEvent::Routed(action_name), 0).expect("");
                },
                OrchestratorEvent::ErrFailedToRoute => {
                    debug!("Failed to route action");
                    msg::reply(OrchestratorEvent::ErrFailedToRoute, 0).expect("");
                },
            }
        },
    }
}

/* #[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
} */


fn init_routes() {
    //let account_contract_id: ActorId = "0xab829a79d6fc14ae39d7dbba09c7f63dde56b17a4c503c20268f9d9ca8c72229";
    let account_contract_id: ActorId = decode("ab829a79d6fc14ae39d7dbba09c7f63dde56b17a4c503c20268f9d9ca8c72229").into();
    unsafe {
        // TODO: here we hardcode the routes for now, 3 should be replaced by the Account contract
        ORCHESTRATOR.as_mut().unwrap().routes.insert(String::from("wallet_login"), account_contract_id);
    }
}

fn decode(hex_string: &str) -> [u8; 32] {
    let mut array = [0; 32];
    for (i, bytes) in hex_string.as_bytes().chunks(2).enumerate() {
        let high = hex_to_u8(bytes[0] as char);
        let low = hex_to_u8(bytes[1] as char);
        array[i] = (high << 4) | low;
    }
    array
}

fn hex_to_u8(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - b'0',
        'a'..='f' => c as u8 - b'a' + 10,
        'A'..='F' => c as u8 - b'A' + 10,
        _ => panic!("invalid hex char"),
    }
}
