use justino_lsp::handlers::publish_diagnostics;
use justino_lsp::state::WorkspaceState;

#[test]
fn test_multilingual_i18n_diagnostics_formatting() {
    let mut state_pt = WorkspaceState::new("pt-BR");
    state_pt.set_document("file:///test.jucode", "fn invalid_code( { return 1;", 1);
    let diag_pt = publish_diagnostics(&state_pt, "file:///test.jucode");

    assert!(!diag_pt.is_empty());
    assert!(diag_pt[0].message.contains("Erro de Sintaxe"));

    let mut state_en = WorkspaceState::new("en-US");
    state_en.set_document("file:///test.jucode", "fn invalid_code( { return 1;", 1);
    let diag_en = publish_diagnostics(&state_en, "file:///test.jucode");

    assert!(!diag_en.is_empty());
    assert!(diag_en[0].message.contains("Syntax Error"));
}
