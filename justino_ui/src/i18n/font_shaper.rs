//! UTF-8 Font Shaping and Text Measurement.

use crate::layout::box_model::Rect;

/// Shaped text run information.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapedText {
    pub text: String,
    pub font_size: f32,
    pub width: f32,
    pub height: f32,
    pub line_count: usize,
}

/// Font shaper measuring text bounding boxes for internationalized strings.
pub struct FontShaper;

impl FontShaper {
    /// Measures dimensions of a text string based on font size.
    pub fn measure_text(text: &str, font_size: f32, max_width: Option<f32>) -> ShapedText {
        let char_count = text.chars().count();
        let avg_char_width = font_size * 0.55;
        let line_height = font_size * 1.2;

        let total_unwrapped_width = char_count as f32 * avg_char_width;

        let (width, line_count) = if let Some(max_w) = max_width {
            if total_unwrapped_width > max_w && max_w > 0.0 {
                let lines = (total_unwrapped_width / max_w).ceil() as usize;
                (max_w, lines.max(1))
            } else {
                (total_unwrapped_width, 1)
            }
        } else {
            (total_unwrapped_width, 1)
        };

        ShapedText {
            text: text.to_string(),
            font_size,
            width,
            height: line_count as f32 * line_height,
            line_count,
        }
    }

    /// Calculates bounding rectangle for text inside a container.
    pub fn bounding_rect(text: &str, font_size: f32, x: f32, y: f32) -> Rect {
        let shaped = Self::measure_text(text, font_size, None);
        Rect::new(x, y, shaped.width, shaped.height)
    }
}
