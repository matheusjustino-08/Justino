//! Button Widget (`button`).

use crate::css::stylesheet::PseudoClass;
use crate::layout::node::UiNode;

/// Interactive Button Widget with state tracking.
pub struct ButtonWidget {
    pub node: UiNode,
    pub is_hovered: bool,
    pub is_active: bool,
}

impl ButtonWidget {
    pub fn new(id: usize, label: impl Into<String>, classes: Vec<String>) -> Self {
        let mut btn_node = UiNode::element(id, "button", classes);
        btn_node.add_child(UiNode::text(id + 1000, label));

        Self {
            node: btn_node,
            is_hovered: false,
            is_active: false,
        }
    }

    pub fn set_hover(&mut self, hover: bool) {
        self.is_hovered = hover;
        self.node.pseudo_state = if hover {
            Some(PseudoClass::Hover)
        } else {
            None
        };
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
        self.node.pseudo_state = if active {
            Some(PseudoClass::Active)
        } else {
            None
        };
    }
}
