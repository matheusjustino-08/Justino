//! Native Window Manager and UI Loop Coordinator.

use crate::css::stylesheet::Stylesheet;
use crate::error::UiError;
use crate::i18n::locale::Locale;
use crate::layout::flexbox::FlexboxEngine;
use crate::layout::node::UiNode;
use crate::render::context::RenderContext;
use crate::render::painter::Painter;
use std::collections::HashMap;

pub type EventCallback = Box<dyn FnMut(usize) -> Result<(), UiError>>;

/// Native OS Window Manager coordinating Layout, Styling, Rendering and Events.
pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub locale: Locale,
    pub stylesheet: Stylesheet,
    pub root_node: UiNode,
    pub render_context: RenderContext,
    pub click_listeners: HashMap<usize, EventCallback>,
}

impl Window {
    pub fn new(title: impl Into<String>, width: u32, height: u32, locale_tag: &str) -> Self {
        let title_str = title.into();
        let locale = Locale::parse(locale_tag);
        let root_node = UiNode::element(1, "window", vec!["window-root".to_string()]);
        let render_context = RenderContext::new(width, height);

        Self {
            title: title_str,
            width,
            height,
            locale,
            stylesheet: Stylesheet::default(),
            root_node,
            render_context,
            click_listeners: HashMap::new(),
        }
    }

    /// Dynamically switches the active locale and recalculates Bidi LTR/RTL layout.
    pub fn set_locale(&mut self, locale_tag: &str) {
        self.locale = Locale::parse(locale_tag);
        self.recalculate_layout();
    }

    /// Updates the window stylesheet.
    pub fn set_stylesheet(&mut self, stylesheet: Stylesheet) {
        self.stylesheet = stylesheet;
        self.recalculate_layout();
    }

    /// Sets the root UI node tree.
    pub fn set_root(&mut self, root: UiNode) {
        self.root_node = root;
        self.recalculate_layout();
    }

    /// Registers a click event listener for a specific node ID.
    pub fn on_click<F>(&mut self, node_id: usize, callback: F)
    where
        F: FnMut(usize) -> Result<(), UiError> + 'static,
    {
        self.click_listeners.insert(node_id, Box::new(callback));
    }

    /// Recalculates spatial layout using Flexbox and Bidi engines.
    pub fn recalculate_layout(&mut self) {
        FlexboxEngine::compute_layout(
            &mut self.root_node,
            &self.stylesheet,
            &self.locale,
            self.width as f32,
            self.height as f32,
        );
    }

    /// Renders a frame into the GPU render context buffer.
    pub fn render_frame(&mut self) -> &RenderContext {
        self.render_context.clear();
        Painter::paint(&self.root_node, &mut self.render_context, self.locale.direction);
        &self.render_context
    }

    /// Dispatches a click event at coordinate (x, y) performing spatial hit-testing.
    pub fn dispatch_click(&mut self, x: f32, y: f32) -> Result<Option<usize>, UiError> {
        let hit_node_id = find_hit_node(&self.root_node, x, y);
        if let Some(id) = hit_node_id {
            if let Some(cb) = self.click_listeners.get_mut(&id) {
                cb(id)?;
            }
        }
        Ok(hit_node_id)
    }

    /// Launches a native desktop GUI window interface.
    pub fn run_native_window(&mut self) -> Result<(), UiError> {
        self.recalculate_layout();
        self.render_frame();
        
        #[cfg(target_os = "windows")]
        {
            if let Err(e) = crate::widget::native_window::launch_win32_window(self) {
                eprintln!("Win32 Window Error: {}", e);
            }
        }

        Ok(())
    }
}

fn find_hit_node(node: &UiNode, px: f32, py: f32) -> Option<usize> {
    if node.dimensions.content.contains_point(px, py) {
        // Search children first for deepest hit
        for child in node.children.iter().rev() {
            if let Some(hit_id) = find_hit_node(child, px, py) {
                return Some(hit_id);
            }
        }
        return Some(node.id);
    }
    None
}
