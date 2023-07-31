#![no_std]
use gstd::{msg, prelude::*, debug};
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
        OrchestratorAction::Route(action_name) => {
            debug!("Routing action: {}", action_name.clone());
            match orchestrator.route(action_name) {
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
    unsafe {
        // TODO: here we hardcode the routes for now, 3 should be replaced by the Account contract
        ORCHESTRATOR.as_mut().unwrap().routes.insert(String::from("wallet_login"), 3.into());
    }
}