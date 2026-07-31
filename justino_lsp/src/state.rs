//! Workspace State and AST Cache Manager for LSP.

use crate::i18n::LspCatalog;
use justino_core::parser::ast::Program;
use justino_core::{Parser, Scanner};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DocumentState {
    pub uri: String,
    pub version: u64,
    pub content: String,
    pub ast: Option<Program>,
}

pub struct WorkspaceState {
    pub documents: HashMap<String, DocumentState>,
    pub css_classes: Vec<String>,
    pub catalog: LspCatalog,
}

impl WorkspaceState {
    pub fn new(locale: &str) -> Self {
        Self {
            documents: HashMap::new(),
            css_classes: Vec::new(),
            catalog: LspCatalog::new(locale),
        }
    }

    pub fn set_document(&mut self, uri: impl Into<String>, content: impl Into<String>, version: u64) {
        let uri_str = uri.into();
        let content_str = content.into();

        // Attempt incremental parse
        let ast = parse_source(&content_str);

        let doc = DocumentState {
            uri: uri_str.clone(),
            version,
            content: content_str,
            ast,
        };

        self.documents.insert(uri_str, doc);
    }

    pub fn set_css_content(&mut self, css_text: &str) {
        let mut classes = Vec::new();
        for line in css_text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('.') {
                let class_name = trimmed
                    .trim_start_matches('.')
                    .split(|c: char| c == '{' || c == ':' || c.is_whitespace())
                    .next()
                    .unwrap_or("")
                    .to_string();
                if !class_name.is_empty() && !classes.contains(&class_name) {
                    classes.push(class_name);
                }
            }
        }
        self.css_classes = classes;
    }
}

fn parse_source(source: &str) -> Option<Program> {
    let mut scanner = Scanner::new(source, 1);
    let tokens = scanner.scan().ok()?;
    let mut parser = Parser::new(tokens, 1);
    parser.parse_program().ok()
}
