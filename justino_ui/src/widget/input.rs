//! Interactive Text Input Widget (`input`).

use crate::css::stylesheet::PseudoClass;
use crate::layout::node::UiNode;

/// Interactive Text Input Box Widget.
pub struct InputWidget {
    pub node: UiNode,
    pub text_content: String,
    pub placeholder: String,
    pub is_focused: bool,
}

impl InputWidget {
    pub fn new(id: usize, placeholder: impl Into<String>, classes: Vec<String>) -> Self {
        let p_str = placeholder.into();
        let mut input_node = UiNode::element(id, "input", classes);
        input_node.add_child(UiNode::text(id + 2000, p_str.clone()));

        Self {
            node: input_node,
            text_content: String::new(),
            placeholder: p_str,
            is_focused: false,
        }
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text_content = text.into();
        let display_text = if self.text_content.is_empty() {
            &self.placeholder
        } else {
            &self.text_content
        };

        if let Some(child) = self.node.children.first_mut() {
            *child = UiNode::text(child.id, display_text.clone());
        }
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
        self.node.pseudo_state = if focused {
            Some(PseudoClass::Focus)
        } else {
            None
        };
    }
}
