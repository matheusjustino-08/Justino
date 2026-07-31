//! Hover Documentation Handler for LSP (`handlers/hover.rs`).

use crate::state::WorkspaceState;

pub fn get_hover_info(state: &WorkspaceState, uri: &str, symbol: &str) -> Option<String> {
    let doc = state.documents.get(uri)?;

    if symbol == "window.create" {
        return Some("```justino\nfn window.create(options: Object) -> WindowInstance\n```\nCreates a GPU-accelerated native desktop app window.".to_string());
    } else if symbol == "http.listen" {
        return Some("```justino\nfn http.listen(port: int, handler: Fn) -> void\n```\nStarts an asynchronous non-blocking HTTP server.".to_string());
    } else if symbol == "db.query" {
        return Some("```justino\nfn db.query(sql: string, params: Array) -> Array<Object>\n```\nExecutes a prepared SQL query against the embedded SQLite database with SQL injection protection.".to_string());
    } else if symbol == "i18n.format_currency" {
        return Some("```justino\nfn i18n.format_currency(amount: float, currency_code: string) -> string\n```\nFormats amount as CLDR currency (e.g. BRL -> 'R$ 1.250,50').".to_string());
    }

    if let Some(ref ast) = doc.ast {
        for stmt in &ast.stmts {
            let stmt_str = format!("{:?}", stmt);
            if stmt_str.contains(symbol) {
                return Some(format!("```justino\n// Symbol definition\n{}\n```", symbol));
            }
        }
    }

    None
}
