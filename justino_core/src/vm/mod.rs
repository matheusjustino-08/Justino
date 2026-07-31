//! Virtual Machine module for executing Justino bytecode.

pub mod gc;
pub mod value;
pub mod vm;

pub use gc::GcArena;
pub use value::Value;
pub use vm::{CallFrame, VM};
