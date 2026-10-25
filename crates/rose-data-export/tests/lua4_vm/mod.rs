//! The Lua 4 VM the game ran NPC dialog scripts with before they became blocks, kept to check
//! the blocks do the same (`dialogs_round_trip.rs`). One change: like Lua, a global the script
//! never set reads as nil (the game's VM stopped the script).

#![allow(dead_code)]

mod value;
mod vm;

pub use rose_file_readers::lua4::{Lua4Function, Lua4Instruction};
pub use value::Lua4Value;
pub use vm::{Lua4VM, Lua4VMError, Lua4VMRustClosures};
