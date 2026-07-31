use justino_lsp::handlers::get_completions;
use justino_lsp::state::WorkspaceState;

#[test]
fn test_completion_keywords_stdlib_and_css_classes() {
    let mut state = WorkspaceState::new("en-US");
    state.set_css_content(".title-app { font-size: 24px; }\n.btn-primary { color: #fff; }");

    let completions = get_completions(&state, "file:///app.jucode", "");

    // Keywords check
    assert!(completions.iter().any(|c| c.label == "fn"));
    assert!(completions.iter().any(|c| c.label == "struct"));

    // Stdlib check
    assert!(completions.iter().any(|c| c.label == "http.listen"));
    assert!(completions.iter().any(|c| c.label == "db.query"));
    assert!(completions.iter().any(|c| c.label == "window.create"));

    // CSS Mapped class check
    assert!(completions.iter().any(|c| c.label == ".title-app"));
    assert!(completions.iter().any(|c| c.label == ".btn-primary"));
}
