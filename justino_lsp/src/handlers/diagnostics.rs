//! Real-Time i18n Diagnostic Error Emitter for LSP (`handlers/diagnostics.rs`).

use crate::protocol::LspDiagnosticItem;
use crate::state::WorkspaceState;
use justino_core::{JustinoError, Parser, Scanner};

pub fn publish_diagnostics(state: &WorkspaceState, uri: &str) -> Vec<LspDiagnosticItem> {
    let mut diagnostics = Vec::new();

    let doc = match state.documents.get(uri) {
        Some(d) => d,
        None => return diagnostics,
    };

    // Incremental parse check
    let res = parse_check(&doc.content);
    if let Err(err) = res {
        let (line, col, msg_key, token) = match &err {
            JustinoError::LexError { message, span } => (span.line as u32, span.column as u32, "unexpected_token", message.clone()),
            JustinoError::ParseError { message, span } => (span.line as u32, span.column as u32, "unexpected_token", message.clone()),
            _ => (1, 1, "unexpected_token", err.to_string()),
        };

        let msg = state.catalog.format(msg_key, line, col, "", &token);

        diagnostics.push(LspDiagnosticItem {
            line,
            character: col,
            severity: 1, // Error
            message: msg,
        });
    }

    diagnostics
}

fn parse_check(source: &str) -> Result<(), JustinoError> {
    let mut scanner = Scanner::new(source, 1);
    let tokens = scanner.scan()?;
    let mut parser = Parser::new(tokens, 1);
    let _ = parser.parse_program()?;
    Ok(())
}
