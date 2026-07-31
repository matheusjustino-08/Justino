//! Specialized AI Context Extractor (`handlers/context_builder.rs`).
//!
//! Reads project ASTs, .jucode files, .css rules, and ARCHITECTURE.md,
//! consolidating them into an optimized JSON payload for AI models in the Phase 5 IDE.

use crate::state::WorkspaceState;
use std::fs;

pub struct AiContextBuilder;

impl AiContextBuilder {
    /// Consolidates project structure, AST statements, CSS rules, and architecture docs into an AI-ready JSON string.
    pub fn build_project_context_json(state: &WorkspaceState) -> String {
        let mut doc_entries = Vec::new();
        for (uri, doc) in &state.documents {
            let stmt_count = doc.ast.as_ref().map(|a| a.stmts.len()).unwrap_or(0);
            doc_entries.push(format!(
                "{{\"uri\":\"{}\",\"version\":{},\"statements_count\":{}}}",
                uri, doc.version, stmt_count
            ));
        }

        let css_classes_str = state
            .css_classes
            .iter()
            .map(|c| format!("\"{}\"", c))
            .collect::<Vec<String>>()
            .join(",");

        let arch_doc = fs::read_to_string("docs/ARCHITECTURE.md")
            .or_else(|_| fs::read_to_string("../docs/ARCHITECTURE.md"))
            .unwrap_or_else(|_| "Architecture Documentation".to_string());

        let arch_summary = arch_doc.lines().take(5).collect::<Vec<&str>>().join(" ");

        format!(
            "{{\"language\":\"Justino\",\"extension\":\".jucode\",\"documents\":[{}],\"css_selectors\":[{}],\"architecture_summary\":\"{}\"}}",
            doc_entries.join(","),
            css_classes_str,
            arch_summary.replace('"', "\\\"")
        )
    }
}
