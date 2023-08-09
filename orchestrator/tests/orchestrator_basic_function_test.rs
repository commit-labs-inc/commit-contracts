use gtest::{ Log, Program, System };
use parity_scale_codec::Encode;
use orchestrator_io::{ OrchestratorAction, OrchestratorEvent, Payload };
const ORCHESTRATOR_ID: u64 = 1;
const SELF_ID: u64 = 2;
const FAKE_ACTOR_ID: u64 = 3;

#[test]
fn add_route() {
    let sys = System::new();
    init_orchestrator(&sys);
    let program = sys.get_program(ORCHESTRATOR_ID);

    let route = String::from("login");
    let res = program.send(SELF_ID, OrchestratorAction::AddRoute(route.clone(), FAKE_ACTOR_ID.into()));
    let log = Log::builder().dest(SELF_ID).payload(OrchestratorEvent::NewRouteCreated(route.clone(), FAKE_ACTOR_ID.into()));
    assert!(res.contains(&log));
}

#[test]
fn add_route_fail_not_owner() {
    let sys = System::new();
    init_orchestrator(&sys);
    let program = sys.get_program(ORCHESTRATOR_ID);

    let route = String::from("login");
    let res = program.send(FAKE_ACTOR_ID, OrchestratorAction::AddRoute(route.clone(), FAKE_ACTOR_ID.into()));
    let log = Log::builder().dest(FAKE_ACTOR_ID).payload(OrchestratorEvent::NotOwner);
    assert!(res.contains(&log));
}

#[test]
fn delete_route() {
    let sys = System::new();
    init_orchestrator(&sys);
    let program = sys.get_program(ORCHESTRATOR_ID);

    let route = String::from("random");
    program.send(SELF_ID, OrchestratorAction::AddRoute(route.clone(), FAKE_ACTOR_ID.into()));
    let res = program.send(SELF_ID, OrchestratorAction::DeleteRoute(route.clone()));
    let log = Log::builder().dest(SELF_ID).payload(OrchestratorEvent::RouteRemoved(route.clone()));
    assert!(res.contains(&log));
}

#[test]
fn delete_route_fail_not_owner() {
    let sys = System::new();
    init_orchestrator(&sys);
    let program = sys.get_program(ORCHESTRATOR_ID);

    let route = String::from("random");
    program.send(SELF_ID, OrchestratorAction::AddRoute(route.clone(), FAKE_ACTOR_ID.into()));
    let res = program.send(FAKE_ACTOR_ID, OrchestratorAction::DeleteRoute(route.clone()));
    let log = Log::builder().dest(FAKE_ACTOR_ID).payload(OrchestratorEvent::NotOwner);
    assert!(res.contains(&log));
}

#[test]
fn route() {
    let sys = System::new();
    init_orchestrator(&sys);
    let program = sys.get_program(ORCHESTRATOR_ID);

    let route = String::from("login");
    let payload = Payload {
        origin: SELF_ID.into(),
        payload: String::from("Hello World!").into_bytes(),
    }.encode();

    program.send(SELF_ID, OrchestratorAction::AddRoute(route.clone(), FAKE_ACTOR_ID.into()));
    let res = program.send(SELF_ID, OrchestratorAction::Route(SELF_ID.into(), route.clone(), payload.clone()));
    let log = Log::builder().dest(FAKE_ACTOR_ID).payload(payload);
    assert!(res.contains(&log));
}

fn init_orchestrator(sys: &System) {
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(SELF_ID, String::from("Hello Orchestrator Contract!"));
    let log = Log::builder().dest(SELF_ID).payload(String::from("Orchestrator Created!"));
    assert!(res.contains(&log));
}