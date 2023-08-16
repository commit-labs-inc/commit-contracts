use gtest::{ Log, Program, System };
use account_io::{ AccountAction, AccountEvent, Profile, SeekerProfile };

const SELF_ID: u64 = 2;

#[test]
fn new_user_login_success() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send(SELF_ID, String::from("Hello Account Contract!"));
    let res = program.send(SELF_ID, AccountAction::Login {
        role: String::from("seeker"),
    });
    let log = Log::builder().dest(SELF_ID).payload(AccountEvent::LoginSuccess {
        profile: Profile::Seeker(SeekerProfile::default()),
    });
    assert!(res.contains(&log));
}

#[test]
fn existing_user_login_success() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send(SELF_ID, String::from("Hello Account Contract!"));
    program.send(SELF_ID, AccountAction::Login {
        role: String::from("seeker"),
    });
    let res = program.send(SELF_ID, AccountAction::Login {
        role: String::from("seeker"),
    });
    let log = Log::builder().dest(SELF_ID).payload(AccountEvent::LoginSuccess {
        profile: Profile::Seeker(SeekerProfile::default()),
    });
    assert!(res.contains(&log));
}

#[test]
fn publish_success() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send(SELF_ID, String::from("Hello Account Contract!"));
    program.send(SELF_ID, AccountAction::Login {
        role: String::from("provider"),
    });
    program.send(SELF_ID, AccountAction::Record { subject: SELF_ID.into(), action: String::from("publish"), quest_id: String::from("01234567890123456789") });
}

#[test]
fn claim_success() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send(SELF_ID, String::from("Hello Account Contract!"));
    program.send(SELF_ID, AccountAction::Login {
        role: String::from("seeker"),
    });
    program.send(SELF_ID, AccountAction::Record { subject: SELF_ID.into(), action: String::from("claim"), quest_id: String::from("01234567890123456789") });
}

#[test]
fn submit_success() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send(SELF_ID, String::from("Hello Account Contract!"));
    program.send(SELF_ID, AccountAction::Login {
        role: String::from("seeker"),
    });
    program.send(SELF_ID, AccountAction::Record { subject: SELF_ID.into(), action: String::from("submit"), quest_id: String::from("01234567890123456789") });
}

#[test]
fn grade_success() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    program.send(SELF_ID, String::from("Hello Account Contract!"));
    program.send(SELF_ID, AccountAction::Login {
        role: String::from("seeker"),
    });
    program.send(SELF_ID, AccountAction::Record { subject: SELF_ID.into(), action: String::from("grade_accepted"), quest_id: String::from("01234567890123456789") });
}