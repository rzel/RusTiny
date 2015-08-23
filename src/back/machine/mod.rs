//! The RusTiny machine description
//!

mod cconv;
#[macro_use] pub mod instructions;
mod registers;


pub use self::instructions::{Argument, Instruction, Assembly};
pub use self::registers::MachineRegister;


pub type Word = u64;