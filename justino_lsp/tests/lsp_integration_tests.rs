use justino_lsp::handlers::{get_definition_location, get_hover_info, AiContextBuilder};
use justino_lsp::state::WorkspaceState;

#[test]
fn test_lsp_e2e_hover_definition_and_ai_context_builder() {
    let mut state = WorkspaceState::new("en-US");
    state.set_document(
        "file:///app.jucode",
        "fn main() -> int { let x = 10; return x; }",
        1,
    );
    state.set_css_content(".btn-action { color: #fff; }");

    // Hover test
    let hover_info = get_hover_info(&state, "file:///app.jucode", "window.create");
    assert!(hover_info.is_some());
    assert!(hover_info.unwrap().contains("window.create"));

    // Definition test
    let def = get_definition_location(&state, "file:///app.jucode", "main");
    assert!(def.is_some());

    // AI Context Builder test
    let ai_json = AiContextBuilder::build_project_context_json(&state);
    assert!(ai_json.contains("\"language\":\"Justino\""));
    assert!(ai_json.contains("\"btn-action\""));
}
