#![no_std]
use gstd::{msg, prelude::*};
use amber_msgs_metaio::*;

// 1. Pushes the first message to the global 'MESSAGES' vector.
// 2. Sends a reply indicating successful initialization.
#[no_mangle]
extern "C" fn init() {
    let first_message = "Pink Floyd".to_string();
    unsafe { MESSAGES.push(first_message) };

    msg::reply_bytes("Successfully initialized", 0).expect("Failed to send reply message.");
}

// 1. Loads the incoming command from the input payload.
// 2. Pushing new value to the global MESSAGES vector
// 3. Sends a reply indicating successful adding.
#[no_mangle]
extern "C" fn handle() {
    let message_handler: Command = msg::load().expect("Unable to decode Message");

    unsafe { MESSAGES.push(message_handler.content) };
    msg::reply_bytes("Message added successfully.", 0).expect("Failed to send reply message.");
}

// 1. Sends the cloned state of the global 'MESSAGES' vector as a reply.
#[no_mangle]
extern "C" fn state() {
    msg::reply(unsafe { MESSAGES.clone() }, 0).expect("Failed to clone and send a reply state");
}