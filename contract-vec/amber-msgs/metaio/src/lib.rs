#![no_std]
use gmeta::{InOut, Metadata, Out};
use gstd::{prelude::*, Vec};

// Represents the metadata structure for handling incoming commands.
pub struct CommandMetadata;

pub static mut MESSAGES: Vec<String> = Vec::new();

// Represents a command in payload.
// - content: content of the message.
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Command {
    pub content: String,
}

// Implementation of the Metadata trait for the CommandMetadata struct.
impl Metadata for CommandMetadata {
    // taking an input as a command
    type Init = Out<String>;
    // taking an input as a command
    type Handle = InOut<Command, String>;    
    // outputs a vector of 'MESSAGES' without expecting any input.
    type State = Out<Vec<String>>;
    
    type Reply = ();
    type Others = ();
    type Signal = ();
}