//! JSON-RPC 2.0 Message Protocol Encoder and Decoder for LSP.

use crate::error::LspError;

#[derive(Debug, Clone)]
pub struct LspRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params_raw: String,
}

impl LspRequest {
    pub fn parse_jsonrpc(raw_json: &str) -> Result<Self, LspError> {
        let trimmed = raw_json.trim();
        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return Err(LspError::ProtocolError("Invalid JSON-RPC frame payload".to_string()));
        }

        let mut id = 0u64;
        let mut method = String::new();
        let mut params_raw = String::new();

        let inner = &trimmed[1..trimmed.len() - 1];
        for pair in inner.split(',') {
            if let Some((k, v)) = pair.split_once(':') {
                let key = k.trim().trim_matches('"');
                let val = v.trim();
                match key {
                    "id" => id = val.parse::<u64>().unwrap_or(0),
                    "method" => method = val.trim_matches('"').to_string(),
                    "params" => params_raw = val.to_string(),
                    _ => {}
                }
            }
        }

        if method.is_empty() {
            return Err(LspError::ProtocolError("Missing method in JSON-RPC request".to_string()));
        }

        Ok(Self {
            jsonrpc: "2.0".to_string(),
            id,
            method,
            params_raw,
        })
    }
}

#[derive(Debug, Clone)]
pub struct LspResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result_json: String,
}

impl LspResponse {
    pub fn new(id: u64, result_json: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result_json: result_json.into(),
        }
    }

    pub fn to_jsonrpc_string(&self) -> String {
        format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{}}}",
            self.id, self.result_json
        )
    }

    pub fn to_transport_frame(&self) -> String {
        let body = self.to_jsonrpc_string();
        format!("Content-Length: {}\r\n\r\n{}", body.len(), body)
    }
}

#[derive(Debug, Clone)]
pub struct LspDiagnosticItem {
    pub line: u32,
    pub character: u32,
    pub severity: u32, // 1 = Error, 2 = Warning, 3 = Information, 4 = Hint
    pub message: String,
}

impl LspDiagnosticItem {
    pub fn to_lsp_json(&self) -> String {
        format!(
            "{{\"range\":{{\"start\":{{\"line\":{},\"character\":{}}},\"end\":{{\"line\":{},\"character\":{}}}}},\"severity\":{},\"message\":\"{}\"}}",
            self.line, self.character, self.line, self.character + 5, self.severity, self.message
        )
    }
}
