#![no_std]
use gstd::{prelude::*, ActorId};
use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata};
use scale_info::TypeInfo;
use hashbrown::HashMap;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    // the init logic will receive a JSON string from the factory contract contains the quest information
    type Init = In<String>;
    type Handle = InOut<OrchestratorAction,OrchestratorEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = String;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum OrchestratorAction {
    // the action to route an action
    Route(String),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum OrchestratorEvent {
    // the event to indicate an action has been routed
    Routed(String),
    ErrFailedToRoute,
}

pub struct Orchestrator {
    // routes is a map from action (String) to a contract (ActorId)
    pub routes: HashMap<String, ActorId>,
}

impl Orchestrator {

    pub fn route(&mut self, action: String) -> OrchestratorEvent {
        // 1. check if the action is in the routes map
        if !self.routes.contains_key(&action) {
            return OrchestratorEvent::ErrFailedToRoute;
        }
        // 2. route the action to the contract
        let contract_id = self.routes.get(&action).unwrap().to_owned();
        let res = gstd::msg::send(contract_id.clone(), action.clone(), 0);
        if res.is_err() {
            return OrchestratorEvent::ErrFailedToRoute;
        }
        gstd::debug!("Routing action: {} to contract: {:?}", action.clone(), contract_id);
        // 3. return the event
        return OrchestratorEvent::Routed(action);
    }
}