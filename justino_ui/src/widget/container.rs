//! Container Widget (`container` / `div` / `box`).

use crate::layout::node::UiNode;

/// Flexbox Container Widget holding child components.
pub struct ContainerWidget {
    pub node: UiNode,
}

impl ContainerWidget {
    pub fn new(id: usize, classes: Vec<String>) -> Self {
        Self {
            node: UiNode::element(id, "container", classes),
        }
    }

    pub fn with_id(mut self, element_id: impl Into<String>) -> Self {
        self.node = self.node.with_id(element_id);
        self
    }

    pub fn add_child(&mut self, child: UiNode) {
        self.node.add_child(child);
    }
}
