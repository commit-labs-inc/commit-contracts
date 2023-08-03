#![no_std]
use gstd::{msg, prelude::*, debug};
use account_io::{AccountAction, AccountEvent, Accounts};

static mut ACCOUNTS: Option<Accounts> = None;

#[no_mangle]
extern "C" fn init() {
    let _init_info: String = msg::load().expect("Failed to load init message");

    unsafe {
        ACCOUNTS = Some(Accounts {
            accounts: HashMap::new(),
        });
    }

    debug!("account contract initialized!");

    msg::reply(String::from("Account contract initialized!"), 0).expect("Failed to reply init message");
}

#[no_mangle]
extern "C" fn handle() {
    let action: AccountAction = msg::load().expect("Failed to load action message");
    let account = unsafe { ACCOUNTS.as_mut().unwrap() };

    match action {
        AccountAction::Login(user_addr) => {
            match account.login(user_addr) {
                AccountEvent::LoginSuccess => {
                    debug!("Login success!");
                    msg::reply(String::from("Login success!"), 0).expect("Failed to reply login success message");
                },
                AccountEvent::Registered => {
                    debug!("Registered!");
                    msg::reply(String::from("Registered!"), 0).expect("Failed to reply registered message");
                },
                AccountEvent::LoginFailed => {
                    debug!("Login failed!");
                    msg::reply(String::from("Login failed!"), 0).expect("Failed to reply login failed message");
                },
            }
        }
    }
}