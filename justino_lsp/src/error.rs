//! Unified Error Types for the Justino Language Server Protocol (LSP).

use justino_core::JustinoError;
use std::fmt;

#[derive(Debug, Clone)]
pub enum LspError {
    ProtocolError(String),
    DocumentNotFound(String),
    ParseError(String),
    CoreError(JustinoError),
}

impl fmt::Display for LspError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LspError::ProtocolError(msg) => write!(f, "JSON-RPC ProtocolError: {}", msg),
            LspError::DocumentNotFound(uri) => write!(f, "DocumentNotFound: {}", uri),
            LspError::ParseError(msg) => write!(f, "ParseError: {}", msg),
            LspError::CoreError(err) => write!(f, "CoreError: {}", err),
        }
    }
}

impl std::error::Error for LspError {}

impl From<JustinoError> for LspError {
    fn from(err: JustinoError) -> Self {
        LspError::CoreError(err)
    }
}
