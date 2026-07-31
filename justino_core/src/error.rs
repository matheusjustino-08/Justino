//! Unified error types for the Justino language pipeline.

use crate::span::Span;
use std::fmt;

/// Primary error enum returned across all stages of `.jucode` code processing.
#[derive(Debug, Clone, PartialEq)]
pub enum JustinoError {
    /// Lexical analysis error.
    LexError { message: String, span: Span },
    /// Syntax analysis / parsing error.
    ParseError { message: String, span: Span },
    /// Bytecode compilation error.
    CompileError { message: String, span: Span },
    /// Virtual Machine runtime execution error.
    RuntimeError { message: String, span: Option<Span> },
}

impl JustinoError {
    /// Retrieves the span associated with the error, if available.
    pub fn span(&self) -> Option<Span> {
        match self {
            JustinoError::LexError { span, .. } => Some(*span),
            JustinoError::ParseError { span, .. } => Some(*span),
            JustinoError::CompileError { span, .. } => Some(*span),
            JustinoError::RuntimeError { span, .. } => *span,
        }
    }
}

impl fmt::Display for JustinoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JustinoError::LexError { message, span } => {
                write!(f, "[LexError at {}]: {}", span, message)
            }
            JustinoError::ParseError { message, span } => {
                write!(f, "[ParseError at {}]: {}", span, message)
            }
            JustinoError::CompileError { message, span } => {
                write!(f, "[CompileError at {}]: {}", span, message)
            }
            JustinoError::RuntimeError { message, span } => match span {
                Some(sp) => write!(f, "[RuntimeError at {}]: {}", sp, message),
                None => write!(f, "[RuntimeError]: {}", message),
            },
        }
    }
}

impl std::error::Error for JustinoError {}
