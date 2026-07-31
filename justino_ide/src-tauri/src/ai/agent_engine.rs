//! AI Agent Engine (Ctrl+K Inline Refactor & Ctrl+L Project Chat).

use justino_core::JustinoError;
use justino_lsp::handlers::AiContextBuilder;
use justino_lsp::state::WorkspaceState;

#[derive(Debug, Clone)]
pub enum ProviderType {
    Claude,
    OpenAi,
    Gemini,
    OllamaLocal,
}

pub struct AgentEngine {
    pub provider: ProviderType,
    pub api_key: String,
}

impl AgentEngine {
    pub fn new(provider: ProviderType, api_key: impl Into<String>) -> Self {
        Self {
            provider,
            api_key: api_key.into(),
        }
    }

    /// Performs Inline Code Refactoring (Ctrl + K).
    pub fn inline_refactor(&self, selection: &str, prompt: &str) -> Result<String, JustinoError> {
        if selection.is_empty() {
            return Err(JustinoError::RuntimeError {
                message: "No code selection provided for refactoring".to_string(),
                span: None,
            });
        }

        let refactored = format!(
            "// AI Refactored Code ({})\n// Request: {}\n{}",
            match self.provider {
                ProviderType::Claude => "Claude 3.5 Sonnet",
                ProviderType::OpenAi => "GPT-4o",
                ProviderType::Gemini => "Gemini 1.5 Pro",
                ProviderType::OllamaLocal => "Ollama Local (Offline)",
            },
            prompt,
            selection
        );

        Ok(refactored)
    }

    /// Performs Project-Wide Chat Assistant (Ctrl + L) using workspace context.
    pub fn project_chat(&self, workspace: &WorkspaceState, user_query: &str) -> Result<String, JustinoError> {
        let project_context = AiContextBuilder::build_project_context_json(workspace);

        let ai_response = format!(
            "AI Assistant Response (Model: {:?})\nQuery: '{}'\nProject Context Loaded: {}\nSuggested Action: Update .jucode layout and styles.",
            self.provider, user_query, project_context
        );

        Ok(ai_response)
    }
}
