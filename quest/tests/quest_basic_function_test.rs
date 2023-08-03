/*
    This test file tests the basic functionality of the quest contract.
    -claim: career aspirant claims a quest
    -submit: career aspirant submits results to a quest
    -grade: quest provider grades a quest
*/

use gtest::{ Log, Program, System };
use quest_io::{ QuestEvent, QuestAction };
const QUEST_ID: u64 = 1;
const SELF_ID: u64 = 2;
// const NON_EXIST_ID: u64 = 3;

#[test]
fn claim_success() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    let res = program.send(SELF_ID, QuestAction::Claim(String::from("a fake quest id for testing only")));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::Claimed);
    assert!(res.contains(&log));
}

// a claimer cannot claim a quest twice
#[test]
fn claim_fail_double_claim() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    program.send(SELF_ID, QuestAction::Claim(String::from("a fake quest id for testing only")));
    let res = program.send(2, QuestAction::Claim(String::from("a fake quest id for testing only")));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::ErrorClaimerExists);
    assert!(res.contains(&log));
}

// cannot claim a non exists quest
#[test]
fn claim_fail_non_exist_quest() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    let res = program.send(SELF_ID, QuestAction::Claim(String::from("a non exists quest id")));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::UnknownError);
    assert!(res.contains(&log));
}

/* #[test]
fn submit_success() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    program.send(SELF_ID, QuestAction::Claim);
    let res = program.send(SELF_ID, QuestAction::Submit(String::from("submission")));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::Submitted);
    assert!(res.contains(&log));
}

// only exising claimers can submit to a quest
#[test]
fn submit_fail() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    // submit without claim the quest first will fail
    let res = program.send(SELF_ID, QuestAction::Submit(String::from("submission")));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::ErrorSubmitterNotExists);
    assert!(res.contains(&log));
}

#[test]
fn grade_success() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    program.send(SELF_ID, QuestAction::Claim);
    let res = program.send(SELF_ID, QuestAction::Grade(SELF_ID.into(), 100));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::Graded);
    assert!(res.contains(&log));
}

// only quest owner can grade a quest
#[test]
fn grade_fail_not_grader() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    let res = program.send(NON_EXIST_ID, QuestAction::Grade(NON_EXIST_ID.into(), 100));
    let log = Log::builder().dest(NON_EXIST_ID).payload(QuestEvent::ErrorNotQuestOwner);
    assert!(res.contains(&log));
}

#[test]
fn grade_fail_recipient_not_exists() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    let res = program.send(SELF_ID, QuestAction::Grade(NON_EXIST_ID.into(), 100));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::ErrorSubmitterNotExists);
    assert!(res.contains(&log));
} */

fn init_quest(sys: &System) {
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(SELF_ID, String::from("Hello Quest Contract!"));
    let log = Log::builder().dest(SELF_ID).payload(String::from("Quest Created!"));
    assert!(res.contains(&log));
    
}