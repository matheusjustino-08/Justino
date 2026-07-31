//! GPU Graphics Context and Command Buffer primitives.

use crate::css::value::Color;
use crate::layout::box_model::Rect;

/// Low-level GPU/Canvas Draw Command.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    FillRect {
        rect: Rect,
        color: Color,
        border_radius: f32,
        opacity: f32,
    },
    DrawBorder {
        rect: Rect,
        color: Color,
        width: f32,
        border_radius: f32,
    },
    DrawText {
        rect: Rect,
        text: String,
        font_size: f32,
        color: Color,
        align_right: bool,
    },
    DrawShadow {
        rect: Rect,
        color: Color,
        blur_radius: f32,
    },
}

/// GPU/Canvas Graphical Context holding the draw command queue.
#[derive(Debug, Clone)]
pub struct RenderContext {
    pub width: u32,
    pub height: u32,
    pub commands: Vec<DrawCommand>,
}

impl RenderContext {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            commands: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn push_command(&mut self, cmd: DrawCommand) {
        self.commands.push(cmd);
    }
}
