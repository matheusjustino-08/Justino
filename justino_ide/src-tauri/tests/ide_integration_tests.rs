use justino_core::JustinoError;
use justino_ide::ai::{AgentEngine, ProviderType};
use justino_ide::extensions::MarketplaceManager;
use justino_ide::lsp::LspBridge;
use justino_ide::runner::CompilerCli;

#[test]
fn test_ide_full_e2e_workflow() -> Result<(), JustinoError> {
    // 1. LSP initialization
    let mut lsp = LspBridge::new("en-US");
    let lsp_res = lsp.handle_request(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#)?;
    assert!(lsp_res.contains("capabilities"));

    // 2. AI Inline Refactor
    let ai = AgentEngine::new(ProviderType::Claude, "key_123");
    let refactored = ai.inline_refactor("let x = 1;", "Optimize x")?;
    assert!(refactored.contains("Claude 3.5 Sonnet"));

    // 3. Marketplace Theme Loading
    let marketplace = MarketplaceManager::new();
    let dark_css = marketplace.get_theme_css("theme.dark_studio")?;
    assert!(dark_css.contains("--bg-primary"));

    // 4. Compiler Execution
    let eval_res = CompilerCli::run_jucode_source("fn main() -> int { return 100; } return main();")?;
    assert_eq!(eval_res.to_string(), "100");

    Ok(())
}
