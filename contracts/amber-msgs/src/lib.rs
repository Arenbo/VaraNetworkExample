#![no_std]
use gstd::{msg, prelude::*};
use amber_msgs_metaio::*;

// 1. Loads the initial command from the input payload.
// 2. Updates the global 'MESSAGE'.
// 3. Sends a reply indicating successful initialization.
#[no_mangle]
extern "C" fn init() {
    // Load the initial command from the input payload
    let init: Command = msg::load().expect("Unable to decode Message");

    // add first message
    unsafe { MESSAGE = init.content };

    msg::reply_bytes("Successfully initialized", 0).expect("Failed to send reply message.");
}

// 1. Loads the incoming command from the input payload.
// 2. Checking incomming command
// 3. Do actions based on command
// 4. Sends a reply.
#[no_mangle]
extern "C" fn handle() {
    let message_handler: Command = msg::load().expect("Unable to decode Message");

    match message_handler.cmd {
        0 => {
            unsafe { MESSAGE = message_handler.content.to_owned() };
        
            msg::reply_bytes("Message updated successfully.", 0).expect("Failed to send reply message.");
        }
        _ => {
            msg::reply_bytes("Unknown command", 0).expect("Failed to send reply message.");
        },
    }
}

// 1. Sends the state as a reply.
#[no_mangle]
extern "C" fn state() {
    msg::reply(unsafe { MESSAGE.to_owned() }, 0).expect("Failed to send a reply state");
}