//! Unified Standard Library Error Types for Justino.

use justino_core::JustinoError;
use justino_ui::UiError;
use std::fmt;

#[derive(Debug, Clone)]
pub enum StdlibError {
    WindowError(String),
    HttpError(String),
    JsonError(String),
    FsError(String),
    CryptoError(String),
    DbError(String),
    I18nError(String),
    CoreError(JustinoError),
    UiError(UiError),
}

impl fmt::Display for StdlibError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StdlibError::WindowError(msg) => write!(f, "WindowError: {}", msg),
            StdlibError::HttpError(msg) => write!(f, "HttpError: {}", msg),
            StdlibError::JsonError(msg) => write!(f, "JsonError: {}", msg),
            StdlibError::FsError(msg) => write!(f, "FsError: {}", msg),
            StdlibError::CryptoError(msg) => write!(f, "CryptoError: {}", msg),
            StdlibError::DbError(msg) => write!(f, "DbError: {}", msg),
            StdlibError::I18nError(msg) => write!(f, "I18nError: {}", msg),
            StdlibError::CoreError(err) => write!(f, "CoreError: {}", err),
            StdlibError::UiError(err) => write!(f, "UiError: {}", err),
        }
    }
}

impl std::error::Error for StdlibError {}

impl From<JustinoError> for StdlibError {
    fn from(err: JustinoError) -> Self {
        StdlibError::CoreError(err)
    }
}

impl From<UiError> for StdlibError {
    fn from(err: UiError) -> Self {
        StdlibError::UiError(err)
    }
}

impl From<StdlibError> for JustinoError {
    fn from(err: StdlibError) -> Self {
        JustinoError::RuntimeError {
            message: err.to_string(),
            span: None,
        }
    }
}
