//! UI Tree Painter emitting GPU primitives with absolute screen coordinate accumulation.

use crate::css::value::{Color, CssValue};
use crate::i18n::locale::TextDirection;
use crate::layout::box_model::Rect;
use crate::layout::node::{NodeType, UiNode};
use crate::render::context::{DrawCommand, RenderContext};

/// Painter traversing the UI tree and generating GPU draw commands.
pub struct Painter;

impl Painter {
    /// Paints the entire UI Node tree into the target RenderContext.
    pub fn paint(node: &UiNode, ctx: &mut RenderContext, direction: TextDirection) {
        Self::paint_recursive(node, ctx, direction, 0.0, 0.0);
    }

    fn paint_recursive(
        node: &UiNode,
        ctx: &mut RenderContext,
        direction: TextDirection,
        parent_x: f32,
        parent_y: f32,
    ) {
        let opacity = match node.computed_styles.get("opacity") {
            Some(CssValue::Number(op)) => *op,
            _ => 1.0,
        };

        if opacity <= 0.0 {
            return;
        }

        let abs_x = parent_x + node.dimensions.content.x;
        let abs_y = parent_y + node.dimensions.content.y;
        let abs_rect = Rect::new(
            abs_x,
            abs_y,
            node.dimensions.content.width,
            node.dimensions.content.height,
        );

        let bg_color = match node.computed_styles.get("background-color") {
            Some(CssValue::Color(c)) => *c,
            _ => Color::TRANSPARENT,
        };

        let border_radius = match node.computed_styles.get("border-radius") {
            Some(CssValue::Px(r)) => *r,
            _ => 0.0,
        };

        // Emit background fill command if not transparent
        if bg_color.a > 0 {
            ctx.push_command(DrawCommand::FillRect {
                rect: abs_rect,
                color: bg_color,
                border_radius,
                opacity,
            });
        }

        // Emit border command if specified
        let border_width = match node.computed_styles.get("border-width") {
            Some(CssValue::Px(w)) => *w,
            _ => 0.0,
        };

        if border_width > 0.0 {
            let border_color = match node.computed_styles.get("border-color") {
                Some(CssValue::Color(c)) => *c,
                _ => Color::BLACK,
            };
            ctx.push_command(DrawCommand::DrawBorder {
                rect: abs_rect,
                color: border_color,
                width: border_width,
                border_radius,
            });
        }

        // Handle text node rendering
        if let NodeType::Text(ref text_str) = node.node_type {
            let font_size = match node.computed_styles.get("font-size") {
                Some(CssValue::Px(sz)) => *sz,
                _ => 16.0,
            };
            let text_color = match node.computed_styles.get("color") {
                Some(CssValue::Color(c)) => *c,
                _ => Color::BLACK,
            };

            ctx.push_command(DrawCommand::DrawText {
                rect: abs_rect,
                text: text_str.clone(),
                font_size,
                color: text_color,
                align_right: direction == TextDirection::Rtl,
            });
        }

        // Paint children recursively with accumulated absolute parent offset
        for child in &node.children {
            Self::paint_recursive(child, ctx, direction, abs_x, abs_y);
        }
    }
}
