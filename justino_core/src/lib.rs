//! Core engine crate for the Justino programming language (`.jucode`).
//!
//! Includes:
//! - Lexer (UTF-8 Scanner)
//! - Parser (Pratt Parser & AST)
//! - Register Bytecode Compiler
//! - High-Performance Virtual Machine & GC

pub mod compiler;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod span;
pub mod vm;

pub use compiler::Compiler;
pub use error::JustinoError;
pub use lexer::Scanner;
pub use parser::Parser;
pub use span::Span;
pub use vm::{Value, VM};

/// Helper utility to compile and execute `.jucode` source code directly.
pub fn eval_jucode(source: &str, file_id: usize) -> Result<Value, JustinoError> {
    let mut scanner = Scanner::new(source, file_id);
    let tokens = scanner.scan()?;

    let mut parser = Parser::new(tokens, file_id);
    let program = parser.parse_program()?;

    let compiler = Compiler::new(file_id);
    let compiled_func = compiler.compile_program(&program)?;

    let mut vm = VM::new();
    vm.interpret(compiled_func)
}
