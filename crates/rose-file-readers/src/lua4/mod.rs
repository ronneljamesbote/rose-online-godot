//! Compiled Lua 4 scripts (the NPC dialog scripts inside a CON file): the functions and their
//! instructions, read but not run.

mod function;
mod instruction;

pub use function::{Lua4Function, Lua4LocalVar};
pub use instruction::Lua4Instruction;
