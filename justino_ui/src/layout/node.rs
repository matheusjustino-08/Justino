//! UI Tree Node representations.

use crate::css::stylesheet::PseudoClass;
use crate::css::value::CssValue;
use crate::layout::box_model::BoxDimensions;
use std::collections::HashMap;

/// Differentiates between element containers and text leaf nodes.
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Element {
        tag: String,
        id: Option<String>,
        classes: Vec<String>,
    },
    Text(String),
}

/// A node in the UI Tree.
#[derive(Debug, Clone, PartialEq)]
pub struct UiNode {
    pub id: usize,
    pub node_type: NodeType,
    pub specified_styles: HashMap<String, CssValue>,
    pub computed_styles: HashMap<String, CssValue>,
    pub dimensions: BoxDimensions,
    pub children: Vec<UiNode>,
    pub pseudo_state: Option<PseudoClass>,
}

impl UiNode {
    pub fn element(id: usize, tag: impl Into<String>, classes: Vec<String>) -> Self {
        Self {
            id,
            node_type: NodeType::Element {
                tag: tag.into(),
                id: None,
                classes,
            },
            specified_styles: HashMap::new(),
            computed_styles: HashMap::new(),
            dimensions: BoxDimensions::default(),
            children: Vec::new(),
            pseudo_state: None,
        }
    }

    pub fn text(id: usize, text: impl Into<String>) -> Self {
        Self {
            id,
            node_type: NodeType::Text(text.into()),
            specified_styles: HashMap::new(),
            computed_styles: HashMap::new(),
            dimensions: BoxDimensions::default(),
            children: Vec::new(),
            pseudo_state: None,
        }
    }

    pub fn with_id(mut self, element_id: impl Into<String>) -> Self {
        if let NodeType::Element { ref mut id, .. } = self.node_type {
            *id = Some(element_id.into());
        }
        self
    }

    pub fn add_child(&mut self, child: UiNode) {
        self.children.push(child);
    }
}
