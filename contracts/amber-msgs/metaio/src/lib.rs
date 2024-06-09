#![no_std]
use gmeta::{InOut, Metadata, Out};
use gstd::prelude::*;

// Represents the metadata structure for handling incoming commands.
pub struct CommandMetadata;

pub static mut MESSAGE: String = String::new();

// Represents a command with an command and content.
// - cmd: 0 - update message
// - content: content of the message.
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Command {
    pub cmd: u64,
    pub content: String,
}

// Implementation of the Metadata trait for the CommandMetadata struct.
impl Metadata for CommandMetadata {
    // taking an input as a command
    type Init = InOut<Command, String>;
    // taking an input as a command
    type Handle = InOut<Command, String>;    
    // outputs a string 'Message' without expecting any input.
    type State = Out<String>;
    
    type Reply = ();
    type Others = ();
    type Signal = ();
}