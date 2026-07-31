//! BCP 47 Locale management and text direction detection.

use std::fmt;

/// Text writing direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextDirection {
    Ltr,
    Rtl,
}

/// Represents a BCP 47 Language Tag (e.g., "pt-BR", "en-US", "ar-SA", "he-IL").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Locale {
    pub tag: String,
    pub language: String,
    pub region: Option<String>,
    pub direction: TextDirection,
}

impl Locale {
    pub fn parse(tag_str: &str) -> Self {
        let tag = tag_str.trim().to_string();
        let parts: Vec<&str> = tag.split('-').collect();
        let language = parts.first().copied().unwrap_or("en").to_lowercase();
        let region = parts.get(1).map(|r| r.to_uppercase());

        // Check if language code belongs to RTL script families
        let direction = match language.as_str() {
            "ar" | "he" | "fa" | "ur" | "syr" | "dv" => TextDirection::Rtl,
            _ => TextDirection::Ltr,
        };

        Self {
            tag,
            language,
            region,
            direction,
        }
    }

    pub fn is_rtl(&self) -> bool {
        self.direction == TextDirection::Rtl
    }

    pub fn pt_br() -> Self {
        Self::parse("pt-BR")
    }

    pub fn ar_sa() -> Self {
        Self::parse("ar-SA")
    }

    pub fn en_us() -> Self {
        Self::parse("en-US")
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:?})", self.tag, self.direction)
    }
}
