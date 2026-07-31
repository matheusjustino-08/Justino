//! Error types for the Justino UI engine.

use justino_core::JustinoError;
use std::fmt;

/// Primary error type returned by `justino_ui` modules.
#[derive(Debug, Clone, PartialEq)]
pub enum UiError {
    /// CSS parsing error.
    CssParseError {
        message: String,
        line: usize,
        column: usize,
    },
    /// Layout computation failure.
    LayoutError { message: String },
    /// Render context or draw call failure.
    RenderError { message: String },
    /// Widget interaction or event handling failure.
    WidgetError { message: String },
    /// Wrapping underlying Justino core error.
    CoreError(JustinoError),
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiError::CssParseError { message, line, column } => {
                write!(f, "[CSS ParseError at {}:{}]: {}", line, column, message)
            }
            UiError::LayoutError { message } => write!(f, "[LayoutError]: {}", message),
            UiError::RenderError { message } => write!(f, "[RenderError]: {}", message),
            UiError::WidgetError { message } => write!(f, "[WidgetError]: {}", message),
            UiError::CoreError(err) => write!(f, "[CoreError]: {}", err),
        }
    }
}

impl std::error::Error for UiError {}

impl From<JustinoError> for UiError {
    fn from(err: JustinoError) -> Self {
        UiError::CoreError(err)
    }
}
