//! Bi-Directional (Bidi LTR <-> RTL) Layout Mirroring Engine.

use crate::css::value::{AlignItems, FlexDirection, JustifyContent};
use crate::i18n::locale::TextDirection;
use crate::layout::box_model::{EdgeSizes, Rect};

/// Bi-directional Layout Mirroring Engine.
pub struct BidiEngine;

impl BidiEngine {
    /// Mirrors flex direction for RTL locales.
    pub fn mirror_flex_direction(dir: FlexDirection, direction: TextDirection) -> FlexDirection {
        if direction == TextDirection::Rtl {
            match dir {
                FlexDirection::Row => FlexDirection::RowReverse,
                FlexDirection::RowReverse => FlexDirection::Row,
                other => other,
            }
        } else {
            dir
        }
    }

    /// Mirrors justify content alignment for RTL locales.
    pub fn mirror_justify_content(justify: JustifyContent, direction: TextDirection) -> JustifyContent {
        if direction == TextDirection::Rtl {
            match justify {
                JustifyContent::FlexStart => JustifyContent::FlexEnd,
                JustifyContent::FlexEnd => JustifyContent::FlexStart,
                other => other,
            }
        } else {
            justify
        }
    }

    /// Mirrors align items alignment for RTL locales.
    pub fn mirror_align_items(align: AlignItems, direction: TextDirection) -> AlignItems {
        if direction == TextDirection::Rtl {
            match align {
                AlignItems::FlexStart => AlignItems::FlexEnd,
                AlignItems::FlexEnd => AlignItems::FlexStart,
                other => other,
            }
        } else {
            align
        }
    }

    /// Mirrors horizontal margins/paddings edge sizes for RTL locales.
    pub fn mirror_edges(edges: EdgeSizes, direction: TextDirection) -> EdgeSizes {
        if direction == TextDirection::Rtl {
            EdgeSizes {
                top: edges.top,
                right: edges.left,
                bottom: edges.bottom,
                left: edges.right,
            }
        } else {
            edges
        }
    }

    /// Computes mirrored horizontal position `x` within container width.
    pub fn mirror_child_rect(child: Rect, container_width: f32, direction: TextDirection) -> Rect {
        if direction == TextDirection::Rtl {
            let mirrored_x = container_width - (child.x + child.width);
            Rect::new(mirrored_x, child.y, child.width, child.height)
        } else {
            child
        }
    }
}
