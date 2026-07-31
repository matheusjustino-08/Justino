//! Compiler module for translating Justino ASTs to register bytecode.

pub mod compiler;
pub mod opcode;

pub use compiler::Compiler;
pub use opcode::{CompiledFunction, Opcode};
