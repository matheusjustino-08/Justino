//! GPU-Accelerated Declarative UI, CSS3 & i18n Engine for Justino (`.jucode`).

pub mod css;
pub mod error;
pub mod i18n;
pub mod layout;
pub mod render;
pub mod widget;

pub use css::{CssParser, Stylesheet};
pub use error::UiError;
pub use i18n::{BidiEngine, Locale, TextDirection};
pub use layout::{BoxDimensions, FlexboxEngine, Rect, UiNode};
pub use render::{DrawCommand, Painter, RenderContext};
pub use widget::{ButtonWidget, ContainerWidget, InputWidget, TextWidget, Window};
