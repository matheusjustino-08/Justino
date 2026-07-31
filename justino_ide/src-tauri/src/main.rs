//! Tauri v2 Desktop IDE Main Entry Point and IPC Registry.

pub mod ai;
pub mod auth;
pub mod extensions;
pub mod lsp;
pub mod runner;

fn main() {
    println!("Justino Studio IDE (.jucode) - Desktop Backend Initialized.");
}

#[cfg(test)]
mod tests {
    use crate::ai::{AgentEngine, ProviderType};
    use crate::auth::OAuthClient;
    use crate::extensions::MarketplaceManager;
    use crate::lsp::LspBridge;
    use crate::runner::CompilerCli;

    #[test]
    fn test_ide_backend_modules_initialization() {
        let auth = OAuthClient::new();
        let (url, _verifier) = auth.generate_pkce_login_url();
        assert!(url.contains("response_type=code"));

        let mut lsp = LspBridge::new("en-US");
        let lsp_res = lsp.handle_request(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#);
        assert!(lsp_res.is_ok());

        let ai = AgentEngine::new(ProviderType::Claude, "test_api_key");
        let refactored = ai.inline_refactor("let x = 10;", "Optimize variable");
        assert!(refactored.is_ok());

        let marketplace = MarketplaceManager::new();
        let dark_css = marketplace.get_theme_css("theme.dark_studio");
        assert!(dark_css.is_ok());

        let run_res = CompilerCli::run_jucode_source("fn main() -> int { return 42; } return main();");
        assert!(run_res.is_ok());
    }
}
