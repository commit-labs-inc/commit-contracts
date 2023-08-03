use gtest::{ Log, Program, System };
use account_io::{ AccountAction, AccountEvent };

const SELF_ID: u64 = 2;

#[test]
fn register_success() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send(SELF_ID, String::from("Hello Account Contract!"));
    let res = program.send(SELF_ID, AccountAction::Login(String::from("2")));
    let log = Log::builder().dest(SELF_ID).payload(AccountEvent::Registered);
    assert!(res.contains(&log));
}

#[test]
fn login_success() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send(SELF_ID, String::from("Hello Account Contract!"));
    program.send(SELF_ID, AccountAction::Login(String::from("2")));
    let res = program.send(SELF_ID, AccountAction::Login(String::from("2")));
    let log = Log::builder().dest(SELF_ID).payload(AccountEvent::LoginSuccess);
    assert!(res.contains(&log));
}