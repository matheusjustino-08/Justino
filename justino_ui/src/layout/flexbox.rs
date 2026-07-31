//! Flexbox Layout Solver in Rust.

use crate::css::stylesheet::Stylesheet;
use crate::css::value::*;
use crate::i18n::bidi::BidiEngine;
use crate::i18n::font_shaper::FontShaper;
use crate::i18n::locale::Locale;
use crate::layout::box_model::{BoxDimensions, EdgeSizes, Rect};
use crate::layout::node::{NodeType, UiNode};

/// Flexbox Layout Solver Engine.
pub struct FlexboxEngine;

impl FlexboxEngine {
    /// Computes spatial layout for a UI Node tree recursively.
    pub fn compute_layout(
        node: &mut UiNode,
        stylesheet: &Stylesheet,
        locale: &Locale,
        parent_width: f32,
        parent_height: f32,
    ) {
        // 1. Resolve cascaded styles for node
        let (tag, id_str, classes) = match &node.node_type {
            NodeType::Element { tag, id, classes } => (tag.as_str(), id.as_deref(), classes.as_slice()),
            NodeType::Text(_) => ("text", None, [].as_slice()),
        };

        node.computed_styles = stylesheet.compute_style(
            tag,
            id_str,
            classes,
            node.pseudo_state.as_ref(),
            &locale.tag,
        );

        // Merge explicit specified styles
        for (k, v) in &node.specified_styles {
            node.computed_styles.insert(k.clone(), v.clone());
        }

        // 2. Resolve Margins & Paddings with Bidi Mirroring
        let font_size = match node.computed_styles.get("font-size") {
            Some(CssValue::Px(sz)) => *sz,
            _ => 16.0,
        };

        let raw_padding = EdgeSizes {
            top: get_length_prop(&node.computed_styles, "padding-top", "padding", parent_width, font_size),
            right: get_length_prop(&node.computed_styles, "padding-right", "padding", parent_width, font_size),
            bottom: get_length_prop(&node.computed_styles, "padding-bottom", "padding", parent_width, font_size),
            left: get_length_prop(&node.computed_styles, "padding-left", "padding", parent_width, font_size),
        };
        let padding = BidiEngine::mirror_edges(raw_padding, locale.direction);

        let raw_margin = EdgeSizes {
            top: get_length_prop(&node.computed_styles, "margin-top", "margin", parent_width, font_size),
            right: get_length_prop(&node.computed_styles, "margin-right", "margin", parent_width, font_size),
            bottom: get_length_prop(&node.computed_styles, "margin-bottom", "margin", parent_width, font_size),
            left: get_length_prop(&node.computed_styles, "margin-left", "margin", parent_width, font_size),
        };
        let margin = BidiEngine::mirror_edges(raw_margin, locale.direction);

        let border_val = get_length_prop(&node.computed_styles, "border-width", "", parent_width, font_size);
        let border = EdgeSizes::all(border_val);

        // 3. Resolve Dimensions
        let explicit_width = get_length_opt(&node.computed_styles, "width", parent_width, font_size);
        let explicit_height = get_length_opt(&node.computed_styles, "height", parent_height, font_size);

        let mut content_width = explicit_width.unwrap_or_else(|| {
            (parent_width - margin.left - margin.right - padding.left - padding.right - border.left - border.right).max(0.0)
        });

        let mut content_height = explicit_height.unwrap_or(0.0);

        // For Text nodes, measure content text bounds
        if let NodeType::Text(ref text_str) = node.node_type {
            let measured = FontShaper::measure_text(text_str, font_size, explicit_width);
            content_width = measured.width;
            content_height = measured.height;
        } else if explicit_width.is_none() || explicit_height.is_none() {
            // Intrinsic fit-content resolution for elements with direct text children (e.g. Buttons, Inputs)
            if let Some(text_child) = node.children.iter().find(|c| matches!(c.node_type, NodeType::Text(_))) {
                if let NodeType::Text(ref text_str) = text_child.node_type {
                    let measured = FontShaper::measure_text(text_str, font_size, None);
                    if explicit_width.is_none() {
                        content_width = (measured.width + padding.left + padding.right + 24.0).max(80.0);
                    }
                    if explicit_height.is_none() {
                        content_height = (measured.height + padding.top + padding.bottom + 12.0).max(36.0);
                    }
                }
            }
        }

        node.dimensions = BoxDimensions {
            content: Rect::new(0.0, 0.0, content_width, content_height),
            padding,
            border,
            margin,
        };

        if node.children.is_empty() {
            return;
        }

        // 4. Flexbox Layout Resolution for Children
        let raw_flex_dir = match node.computed_styles.get("flex-direction") {
            Some(CssValue::Direction(d)) => *d,
            _ => FlexDirection::Column,
        };
        let flex_dir = BidiEngine::mirror_flex_direction(raw_flex_dir, locale.direction);

        let raw_justify = match node.computed_styles.get("justify-content") {
            Some(CssValue::Justify(j)) => *j,
            Some(CssValue::Keyword(k)) => match k.as_str() {
                "center" => JustifyContent::Center,
                "flex-end" => JustifyContent::FlexEnd,
                "space-between" => JustifyContent::SpaceBetween,
                "space-around" => JustifyContent::SpaceAround,
                "space-evenly" => JustifyContent::SpaceEvenly,
                _ => JustifyContent::FlexStart,
            },
            _ => {
                if tag == "button" || tag == "input" {
                    JustifyContent::Center
                } else {
                    JustifyContent::FlexStart
                }
            }
        };
        let justify = BidiEngine::mirror_justify_content(raw_justify, locale.direction);

        let raw_align = match node.computed_styles.get("align-items") {
            Some(CssValue::Align(a)) => *a,
            Some(CssValue::Keyword(k)) => match k.as_str() {
                "center" => AlignItems::Center,
                "flex-end" => AlignItems::FlexEnd,
                "stretch" => AlignItems::Stretch,
                _ => AlignItems::FlexStart,
            },
            _ => {
                if tag == "button" || tag == "input" {
                    AlignItems::Center
                } else {
                    AlignItems::FlexStart
                }
            }
        };
        let align = BidiEngine::mirror_align_items(raw_align, locale.direction);

        let gap = get_length_prop(&node.computed_styles, "gap", "", content_width, font_size);

        // Recursively compute child dimensions first
        for child in &mut node.children {
            Self::compute_layout(child, stylesheet, locale, content_width, content_height);
        }

        // Position children along Main and Cross axes
        let is_row = matches!(flex_dir, FlexDirection::Row | FlexDirection::RowReverse);

        let total_children_main_size: f32 = node
            .children
            .iter()
            .map(|c| if is_row { c.dimensions.outer_width() } else { c.dimensions.outer_height() })
            .sum();

        let num_gaps = if node.children.len() > 1 { node.children.len() - 1 } else { 0 };
        let total_gap_size = num_gaps as f32 * gap;
        let remaining_main_space = (if is_row { content_width } else { content_height } - total_children_main_size - total_gap_size).max(0.0);

        // Handle flex-grow distribution
        let total_flex_grow: f32 = node
            .children
            .iter()
            .map(|c| match c.computed_styles.get("flex-grow") {
                Some(CssValue::Number(g)) => *g,
                _ => 0.0,
            })
            .sum();

        if total_flex_grow > 0.0 && remaining_main_space > 0.0 {
            for child in &mut node.children {
                let grow = match child.computed_styles.get("flex-grow") {
                    Some(CssValue::Number(g)) => *g,
                    _ => 0.0,
                };
                if grow > 0.0 {
                    let extra = (grow / total_flex_grow) * remaining_main_space;
                    if is_row {
                        child.dimensions.content.width += extra;
                    } else {
                        child.dimensions.content.height += extra;
                    }
                }
            }
        }

        // Compute starting Main Axis offset according to JustifyContent
        let mut main_offset = match justify {
            JustifyContent::FlexStart => 0.0,
            JustifyContent::Center => remaining_main_space / 2.0,
            JustifyContent::FlexEnd => remaining_main_space,
            JustifyContent::SpaceBetween => 0.0,
            JustifyContent::SpaceAround => {
                if !node.children.is_empty() {
                    remaining_main_space / (node.children.len() as f32 * 2.0)
                } else {
                    0.0
                }
            }
            JustifyContent::SpaceEvenly => {
                if !node.children.is_empty() {
                    remaining_main_space / (node.children.len() as f32 + 1.0)
                } else {
                    0.0
                }
            }
        };

        let step_gap = match justify {
            JustifyContent::SpaceBetween => {
                if num_gaps > 0 {
                    gap + (remaining_main_space / num_gaps as f32)
                } else {
                    gap
                }
            }
            JustifyContent::SpaceAround => {
                if !node.children.is_empty() {
                    gap + (remaining_main_space / node.children.len() as f32)
                } else {
                    gap
                }
            }
            JustifyContent::SpaceEvenly => {
                if !node.children.is_empty() {
                    gap + (remaining_main_space / (node.children.len() as f32 + 1.0))
                } else {
                    gap
                }
            }
            _ => gap,
        };

        let is_reverse = matches!(flex_dir, FlexDirection::RowReverse | FlexDirection::ColumnReverse);

        let child_indices: Vec<usize> = if is_reverse {
            (0..node.children.len()).rev().collect()
        } else {
            (0..node.children.len()).collect()
        };

        let mut max_cross_extent: f32 = 0.0;

        for &idx in &child_indices {
            let child = &mut node.children[idx];
            let child_main_size = if is_row { child.dimensions.outer_width() } else { child.dimensions.outer_height() };
            let child_cross_size = if is_row { child.dimensions.outer_height() } else { child.dimensions.outer_width() };

            let container_cross = if is_row { content_height } else { content_width };
            let cross_offset = match align {
                AlignItems::FlexStart => 0.0,
                AlignItems::Center => (container_cross - child_cross_size) / 2.0,
                AlignItems::FlexEnd => container_cross - child_cross_size,
                AlignItems::Stretch => {
                    if is_row && explicit_height.is_none() {
                        child.dimensions.content.height = container_cross;
                    }
                    0.0
                }
            };

            if is_row {
                child.dimensions.content.x = main_offset + child.dimensions.margin.left + child.dimensions.padding.left + child.dimensions.border.left;
                child.dimensions.content.y = cross_offset + child.dimensions.margin.top + child.dimensions.padding.top + child.dimensions.border.top;
            } else {
                child.dimensions.content.x = cross_offset + child.dimensions.margin.left + child.dimensions.padding.left + child.dimensions.border.left;
                child.dimensions.content.y = main_offset + child.dimensions.margin.top + child.dimensions.padding.top + child.dimensions.border.top;
            }

            main_offset += child_main_size + step_gap;
            max_cross_extent = max_cross_extent.max(cross_offset + child_cross_size);
        }

        // If height was auto, set content height to total main/cross extension
        if explicit_height.is_none() {
            if is_row {
                node.dimensions.content.height = max_cross_extent;
            } else {
                node.dimensions.content.height = main_offset;
            }
        }
    }
}

fn get_length_prop(styles: &std::collections::HashMap<String, CssValue>, specific_key: &str, fallback_key: &str, parent_size: f32, font_size: f32) -> f32 {
    if let Some(val) = styles.get(specific_key) {
        val.to_px(parent_size, font_size)
    } else if let Some(val) = styles.get(fallback_key) {
        val.to_px(parent_size, font_size)
    } else {
        0.0
    }
}

fn get_length_opt(styles: &std::collections::HashMap<String, CssValue>, key: &str, parent_size: f32, font_size: f32) -> Option<f32> {
    styles.get(key).map(|v| v.to_px(parent_size, font_size))
}
