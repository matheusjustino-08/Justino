use justino_lsp::error::LspError;
use justino_lsp::protocol::{LspRequest, LspResponse};

#[test]
fn test_json_rpc_request_parsing() -> Result<(), LspError> {
    let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"textDocument/completion","params":{}}"#;

    let req = LspRequest::parse_jsonrpc(raw_json)?;
    assert_eq!(req.id, 1);
    assert_eq!(req.method, "textDocument/completion");

    let response = LspResponse::new(req.id, "{\"status\":\"ok\"}");
    let frame = response.to_transport_frame();

    assert!(frame.contains("Content-Length:"));
    assert!(frame.contains("\"id\":1"));

    Ok(())
}
