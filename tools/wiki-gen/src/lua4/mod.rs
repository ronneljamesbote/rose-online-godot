//! The client's Lua 4 bytecode reader, shared by path so NPC dialog scripts can be read here
//! without the Godot crate.

#[path = "../../../../godot/rust/src/lua4/function.rs"]
#[allow(dead_code)]
mod function;
#[path = "../../../../godot/rust/src/lua4/instruction.rs"]
mod instruction;

pub use function::Lua4Function;
pub use instruction::Lua4Instruction;
