#![no_std]
use gstd::{msg, prelude::*};
use amber_msgs_metaio::*;

// 1. Set initial message to the global 'MESSAGE' string.
// 2. Sends a reply indicating successful initialization.
#[no_mangle]
extern "C" fn init() {
    let initial_message = "Rolling Stones".to_string();
    unsafe { MESSAGE = initial_message };

    msg::reply_bytes("Successfully initialized", 0).expect("Failed to send reply message.");
}

// 1. Loads the incoming command from the input payload.
// 2. Updating global MESSAGE string.
// 3. Sends a reply indicating successful adding.
#[no_mangle]
extern "C" fn handle() {
    let message_handler: Command = msg::load().expect("Unable to decode Message");
    unsafe { MESSAGE = message_handler.content.to_owned() };

    msg::reply_bytes("Message added successfully.", 0).expect("Failed to send reply message.");
}

// 1. Sends the message stored in the global 'MESSAGE' string as a reply.
#[no_mangle]
extern "C" fn state() {
    msg::reply_bytes(unsafe { MESSAGE.to_owned() }, 0).expect("Failed to send a reply state");
}