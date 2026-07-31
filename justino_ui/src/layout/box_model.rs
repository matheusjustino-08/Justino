//! Geometry, Box Model, and Dimension calculations.

/// Absolute rectangle in 2D space.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains_point(&self, px: f32, py: f32) -> bool {
        px >= self.x && px <= self.x + self.width && py >= self.y && py <= self.y + self.height
    }
}

/// 4-sided edge dimensions (Margin, Border, Padding).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EdgeSizes {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeSizes {
    pub fn zero() -> Self {
        Self::default()
    }

    pub fn all(val: f32) -> Self {
        Self {
            top: val,
            right: val,
            bottom: val,
            left: val,
        }
    }

    pub fn symmetric(vertical: f32, horizontal: f32) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }
}

/// Full Box Model geometry.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BoxDimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

impl BoxDimensions {
    /// Total width including content, padding, border, and margin.
    pub fn outer_width(&self) -> f32 {
        self.content.width
            + self.padding.left
            + self.padding.right
            + self.border.left
            + self.border.right
            + self.margin.left
            + self.margin.right
    }

    /// Total height including content, padding, border, and margin.
    pub fn outer_height(&self) -> f32 {
        self.content.height
            + self.padding.top
            + self.padding.bottom
            + self.border.top
            + self.border.bottom
            + self.margin.top
            + self.margin.bottom
    }
}
