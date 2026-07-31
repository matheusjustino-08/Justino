//! Binary Driver Entry Point for 'justino-lsp'.

use justino_lsp::handlers::{get_completions, get_definition_location, get_hover_info};
use justino_lsp::protocol::{LspRequest, LspResponse};
use justino_lsp::state::WorkspaceState;
use std::io::{self, BufRead, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = WorkspaceState::new("en-US");
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut header_line = String::new();
        if handle.read_line(&mut header_line)? == 0 {
            break; // EOF
        }

        if !header_line.starts_with("Content-Length:") {
            continue;
        }

        let len_str = header_line["Content-Length:".len()..].trim();
        let content_len: usize = match len_str.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        // Read empty CRLF line
        let mut empty_line = String::new();
        handle.read_line(&mut empty_line)?;

        // Read JSON payload body
        let mut body_buf = vec![0u8; content_len];
        handle.read_exact(&mut body_buf)?;
        let raw_json = String::from_utf8_lossy(&body_buf);

        if let Ok(req) = LspRequest::parse_jsonrpc(&raw_json) {
            let response = process_request(&mut state, &req);
            let frame = response.to_transport_frame();
            let mut stdout = io::stdout().lock();
            stdout.write_all(frame.as_bytes())?;
            stdout.flush()?;
        }
    }

    Ok(())
}

fn process_request(state: &mut WorkspaceState, req: &LspRequest) -> LspResponse {
    match req.method.as_str() {
        "initialize" => {
            let res_json = r#"{"capabilities":{"textDocumentSync":1,"completionProvider":{"triggerCharacters":[".",":"]},"hoverProvider":true,"definitionProvider":true}}"#;
            LspResponse::new(req.id, res_json)
        }
        "textDocument/completion" => {
            let items = get_completions(state, "file:///app.jucode", "");
            let json_items: Vec<String> = items
                .iter()
                .map(|item| {
                    format!(
                        "{{\"label\":\"{}\",\"kind\":{},\"detail\":\"{}\"}}",
                        item.label, item.kind, item.detail
                    )
                })
                .collect();
            let result_json = format!("{{\"isIncomplete\":false,\"items\":[{}]}}", json_items.join(","));
            LspResponse::new(req.id, result_json)
        }
        "textDocument/hover" => {
            let info = get_hover_info(state, "file:///app.jucode", "window.create").unwrap_or_default();
            let result_json = format!("{{\"contents\":\"{}\"}}", info.replace('\n', "\\n"));
            LspResponse::new(req.id, result_json)
        }
        "textDocument/definition" => {
            let loc = get_definition_location(state, "file:///app.jucode", "main");
            if let Some(l) = loc {
                let result_json = format!(
                    "{{\"uri\":\"{}\",\"range\":{{\"start\":{{\"line\":{},\"character\":{}}},\"end\":{{\"line\":{},\"character\":{}}}}}}}",
                    l.uri, l.line, l.character, l.line, l.character
                );
                LspResponse::new(req.id, result_json)
            } else {
                LspResponse::new(req.id, "null")
            }
        }
        _ => LspResponse::new(req.id, "{}"),
    }
}
