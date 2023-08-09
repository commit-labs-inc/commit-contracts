#![no_std]
use gstd::{msg, prelude::*, debug};
use orchestrator_io::{Orchestrator, OrchestratorAction, OrchestratorEvent};

static mut ORCHESTRATOR: Option<Orchestrator> = None;

#[no_mangle]
extern "C" fn init() {
    let _msg: String = msg::load().expect("Failed to load init info");

    unsafe {
        ORCHESTRATOR = Some(Orchestrator {
            owner: msg::source(),
            routes: HashMap::new(),
        });
    }

    debug!("Orchestrator initiated");
    msg::reply("Orchestrator initiated", 0).expect("Orchestrator initiation failed");
}

#[no_mangle]
extern "C" fn handle() {
    let action: OrchestratorAction = msg::load().expect("Failed to load action");
    let orchestrator = unsafe { ORCHESTRATOR.as_mut().expect("Orchestrator not initiated") };

    match action {
        OrchestratorAction::AddRoute(route, actor_id) => {
            if msg::source() != orchestrator.owner {
                debug!("Warning: non-owner tried to add route!");
                msg::reply(OrchestratorEvent::NotOwner, 0).expect("Failed to emit add route event");
            } else {
                let event = orchestrator.add_route(route, actor_id);
                debug!("Add route event: {:?}", event);
                msg::reply(event, 0).expect("Failed to emit add route event");
            }
        }
        OrchestratorAction::DeleteRoute(route) => {
            if msg::source() != orchestrator.owner {
                debug!("Warning: non-owner tried to delete route!");
                msg::reply(OrchestratorEvent::NotOwner, 0).expect("Failed to emit delete route event");
            } else {
                let event = orchestrator.delete_route(route);
                debug!("Delete route event: {:?}", event);
                msg::reply(event, 0).expect("Failed to emit delete route event");
            }
        }
        OrchestratorAction::Route(origin, route, payload) => {
            let event = orchestrator.route(origin, route, payload);
            debug!("Route event: {:?}", event);
            msg::reply(event, 0).expect("Failed to emit route event");
        }
    }
}