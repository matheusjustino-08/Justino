//! Process Bridge Manager for `justino-lsp`.

use justino_core::JustinoError;
use justino_lsp::protocol::{LspRequest, LspResponse};
use justino_lsp::state::WorkspaceState;

pub struct LspBridge {
    pub workspace_state: WorkspaceState,
}

impl LspBridge {
    pub fn new(locale: &str) -> Self {
        Self {
            workspace_state: WorkspaceState::new(locale),
        }
    }

    pub fn handle_request(&mut self, request_json: &str) -> Result<String, JustinoError> {
        let req = LspRequest::parse_jsonrpc(request_json).map_err(|e| JustinoError::RuntimeError {
            message: e.to_string(),
            span: None,
        })?;

        let res = match req.method.as_str() {
            "initialize" => {
                let capabilities = r#"{"capabilities":{"completionProvider":true,"hoverProvider":true}}"#;
                LspResponse::new(req.id, capabilities)
            }
            "textDocument/completion" => {
                let completions = justino_lsp::handlers::get_completions(&self.workspace_state, "file:///app.jucode", "");
                let items_json: Vec<String> = completions
                    .iter()
                    .map(|c| format!("{{\"label\":\"{}\",\"kind\":{},\"detail\":\"{}\"}}", c.label, c.kind, c.detail))
                    .collect();
                LspResponse::new(req.id, format!("{{\"items\":[{}]}}", items_json.join(",")))
            }
            _ => LspResponse::new(req.id, "{}"),
        };

        Ok(res.to_jsonrpc_string())
    }
}
