//! CSS Property Values and Color representations.

use std::fmt;

/// Represents RGBA color values in CSS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a as f32 / 255.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayValue {
    Flex,
    Block,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignItems {
    FlexStart,
    Center,
    FlexEnd,
    Stretch,
}

/// Represents any parsed CSS property value.
#[derive(Debug, Clone, PartialEq)]
pub enum CssValue {
    Px(f32),
    Rem(f32),
    Em(f32),
    Percent(f32),
    Vh(f32),
    Vw(f32),
    Auto,
    Color(Color),
    Number(f32),
    Display(DisplayValue),
    Direction(FlexDirection),
    Justify(JustifyContent),
    Align(AlignItems),
    Keyword(String),
}

impl CssValue {
    /// Resolves length to concrete pixels given reference context sizes.
    pub fn to_px(&self, parent_size: f32, root_font_size: f32) -> f32 {
        match self {
            CssValue::Px(v) => *v,
            CssValue::Rem(v) => *v * root_font_size,
            CssValue::Em(v) => *v * root_font_size,
            CssValue::Percent(v) => (*v / 100.0) * parent_size,
            CssValue::Vh(v) => (*v / 100.0) * 600.0, // Default viewport height reference
            CssValue::Vw(v) => (*v / 100.0) * 800.0, // Default viewport width reference
            CssValue::Number(v) => *v,
            _ => 0.0,
        }
    }
}
