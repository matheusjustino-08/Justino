//! GPU-Accelerated Native Window View Manager.

use crate::error::StdlibError;
use justino_ui::css::CssParser;
use justino_ui::widget::Window;

pub struct NativeView {
    pub window: Window,
}

impl NativeView {
    pub fn new(title: &str, width: u32, height: u32, locale: &str) -> Self {
        let window = Window::new(title, width, height, locale);
        Self { window }
    }

    pub fn set_stylesheet(&mut self, css_content: &str) -> Result<(), StdlibError> {
        let mut parser = CssParser::new(css_content);
        let stylesheet = parser.parse().map_err(|e| StdlibError::WindowError(e.to_string()))?;
        self.window.set_stylesheet(stylesheet);
        Ok(())
    }

    pub fn launch(&mut self) -> Result<(), StdlibError> {
        self.window.run_native_window().map_err(StdlibError::UiError)
    }
}
