//! Smart Completion Handler for Keywords, Stdlib, and Mapped CSS Selectors (`handlers/completion.rs`).

use crate::state::WorkspaceState;

#[derive(Debug, Clone)]
pub struct CompletionItem {
    pub label: String,
    pub kind: u32, // 1 = Text, 2 = Method, 3 = Function, 6 = Variable, 7 = Class, 14 = Keyword
    pub detail: String,
}

pub fn get_completions(state: &WorkspaceState, _uri: &str, prefix: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // 1. Language Keywords
    let keywords = [
        ("fn", "Function Declaration"),
        ("let", "Variable Binding"),
        ("mut", "Mutable Binding"),
        ("struct", "Struct Type Definition"),
        ("async", "Async Function Modifier"),
        ("await", "Async Await Expression"),
        ("return", "Return Statement"),
        ("if", "Conditional Branch"),
        ("else", "Alternative Branch"),
        ("match", "Pattern Match"),
        ("spawn", "Concurrent Task Spawn"),
        ("import", "Module Import"),
    ];

    for (kw, doc) in keywords {
        if prefix.is_empty() || kw.starts_with(prefix) {
            items.push(CompletionItem {
                label: kw.to_string(),
                kind: 14, // Keyword
                detail: doc.to_string(),
            });
        }
    }

    // 2. Stdlib Builtins
    let stdlib_methods = [
        ("window.create", "Create GPU Native App Window"),
        ("window.set_stylesheet", "Apply CSS stylesheet to Window"),
        ("http.listen", "Start async HTTP server"),
        ("http.get", "Perform HTTP GET request"),
        ("http.post", "Perform HTTP POST request"),
        ("json.parse", "Parse JSON string into native object"),
        ("json.stringify", "Serialize native object to JSON string"),
        ("fs.read_file", "Read UTF-8 text file"),
        ("fs.write_file", "Write UTF-8 text file"),
        ("fs.load_env", "Load .env environment variables"),
        ("crypto.hash_password", "Generate salted password hash"),
        ("crypto.verify_password", "Verify password against hash"),
        ("crypto.sign_jwt", "Sign payload into JWT token"),
        ("db.open", "Open embedded SQLite database"),
        ("db.query", "Execute prepared SQL query"),
        ("i18n.set_locale", "Set active locale"),
        ("i18n.format_currency", "Format amount as CLDR currency"),
    ];

    for (method, doc) in stdlib_methods {
        if prefix.is_empty() || method.starts_with(prefix) {
            items.push(CompletionItem {
                label: method.to_string(),
                kind: 2, // Method
                detail: doc.to_string(),
            });
        }
    }

    // 3. Mapped CSS Selectors
    for css_class in &state.css_classes {
        let class_label = format!(".{}", css_class);
        if prefix.is_empty() || class_label.starts_with(prefix) || css_class.starts_with(prefix) {
            items.push(CompletionItem {
                label: class_label.clone(),
                kind: 7, // Class
                detail: format!("CSS Class from stylesheet ({})", class_label),
            });
        }
    }

    items
}
