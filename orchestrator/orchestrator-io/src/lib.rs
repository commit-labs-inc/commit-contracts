#![no_std]
use gstd::{prelude::*, ActorId, msg};
use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata};
use scale_info::TypeInfo;
use hashbrown::HashMap;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    // the init logic will receive a JSON string from the factory contract contains the quest information
    type Init = In<String>;
    type Handle = InOut<OrchestratorAction, OrchestratorEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = String;
}

pub struct Orchestrator {
    pub owner: ActorId,
    pub routes: HashMap<String, ActorId>,
}

impl Orchestrator {
    pub fn add_route(&mut self, route: String, actor_id: ActorId) -> OrchestratorEvent {
        self.routes.insert(route.clone(), actor_id.clone());
        OrchestratorEvent::NewRouteCreated(route, actor_id)
    }

    pub fn delete_route(&mut self, route: String) -> OrchestratorEvent {
        self.routes.remove(&route);
        OrchestratorEvent::RouteRemoved(route)
    }

    pub fn route(&self, origin: ActorId, route: String, original_payload: Vec<u8>) -> OrchestratorEvent {
        let destination = self.routes.get(&route).expect("route not found");
        // send both the origin's id and the original payload to the destination
        let payload = Payload {
            origin,
            payload: original_payload,
        }.encode();

        if let Ok(_) = msg::send(destination.to_owned(), payload, 0) {
            OrchestratorEvent::RoutingSuccess
        } else {
            OrchestratorEvent::RoutingFailed
        }
    }
}

#[derive(Encode)]
#[codec(crate = gstd::codec)]
pub struct Payload {
    pub origin: ActorId,
    pub payload: Vec<u8>,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum OrchestratorAction {
    AddRoute(String, ActorId), // route (String) to a contract (ActorId)
    DeleteRoute(String),
    Route(ActorId, String, Vec<u8>), // ActorId is the action initiator, String is the route, Vec<u8> is the payload
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum OrchestratorEvent {
    NewRouteCreated(String, ActorId), // route (String) to a contract (ActorId)
    RouteRemoved(String),
    RouteDeleted,
    RoutingSuccess,
    RoutingFailed,
    NotOwner,
}