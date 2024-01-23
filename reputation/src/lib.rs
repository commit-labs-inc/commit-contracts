#![no_std]
use gstd::{ msg, prelude::* };

#[no_mangle]
extern "C" fn init() {
    let _init_info: String = msg::load().expect("Failed to load init info");

    let _ = msg::reply(String::from("Hello, world!"), 0);
}

#[no_mangle]
extern "C" fn handle() {
    
}