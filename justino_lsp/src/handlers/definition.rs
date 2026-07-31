//! Go to Definition Navigation Location Resolver (`handlers/definition.rs`).

use crate::state::WorkspaceState;

#[derive(Debug, Clone)]
pub struct Location {
    pub uri: String,
    pub line: u32,
    pub character: u32,
}

pub fn get_definition_location(state: &WorkspaceState, uri: &str, _symbol: &str) -> Option<Location> {
    let doc = state.documents.get(uri)?;

    if doc.ast.is_some() {
        return Some(Location {
            uri: uri.to_string(),
            line: 1,
            character: 0,
        });
    }

    None
}
