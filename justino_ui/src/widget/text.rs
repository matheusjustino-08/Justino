//! Text Label Widget (`text`).

use crate::layout::node::UiNode;

/// Internationalized Text Label Widget.
pub struct TextWidget {
    pub node: UiNode,
}

impl TextWidget {
    pub fn new(id: usize, text: impl Into<String>) -> Self {
        Self {
            node: UiNode::text(id, text),
        }
    }
}
