//! Internationalization (i18n), BCP 47 Locale and Bi-Directional Layout System.

pub mod bidi;
pub mod font_shaper;
pub mod locale;

pub use bidi::BidiEngine;
pub use font_shaper::{FontShaper, ShapedText};
pub use locale::{Locale, TextDirection};
