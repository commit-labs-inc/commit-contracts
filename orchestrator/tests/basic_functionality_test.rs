use gtest::{ Log, Program, System };
use orchestrator_io::{ OrchestratorAction, OrchestratorEvent };

const SELF_ID: u64 = 2;

#[test]
fn route_success() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send(SELF_ID, String::from("Hello Orchestrator!"));
    let res = program.send(SELF_ID, OrchestratorAction::Route(String::from("wallet_login")));
    let log = Log::builder().dest(SELF_ID).payload(OrchestratorEvent::Routed(String::from("wallet_login")));
    assert!(res.contains(&log));
}
