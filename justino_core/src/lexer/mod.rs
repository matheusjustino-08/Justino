//! Lexer module for tokenizing `.jucode` source files.

pub mod scanner;
pub mod token;

pub use scanner::Scanner;
pub use token::{Token, TokenKind};
