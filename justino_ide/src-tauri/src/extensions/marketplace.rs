//! Extension and Theme Store Marketplace Manager (`extensions/marketplace.rs`).

use justino_core::JustinoError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExtensionItem {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub is_theme: bool,
    pub css_content: Option<String>,
}

pub struct MarketplaceManager {
    pub installed_extensions: HashMap<String, ExtensionItem>,
}

impl MarketplaceManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            installed_extensions: HashMap::new(),
        };
        mgr.init_official_themes();
        mgr
    }

    fn init_official_themes(&mut self) {
        let dark_theme = ExtensionItem {
            id: "theme.dark_studio".to_string(),
            name: "Dark Studio Theme".to_string(),
            version: "1.0.0".to_string(),
            author: "Justino Core Team".to_string(),
            is_theme: true,
            css_content: Some(include_str!("../../../ui/themes/dark_theme.css").to_string()),
        };

        let cyberpunk_theme = ExtensionItem {
            id: "theme.cyberpunk".to_string(),
            name: "Cyberpunk Neon Theme".to_string(),
            version: "1.0.0".to_string(),
            author: "Justino Core Team".to_string(),
            is_theme: true,
            css_content: Some(include_str!("../../../ui/themes/cyberpunk_theme.css").to_string()),
        };

        self.installed_extensions.insert(dark_theme.id.clone(), dark_theme);
        self.installed_extensions.insert(cyberpunk_theme.id.clone(), cyberpunk_theme);
    }

    pub fn get_theme_css(&self, theme_id: &str) -> Result<String, JustinoError> {
        let ext = self
            .installed_extensions
            .get(theme_id)
            .ok_or_else(|| JustinoError::RuntimeError {
                message: format!("Theme '{}' not found", theme_id),
                span: None,
            })?;

        ext.css_content.clone().ok_or_else(|| JustinoError::RuntimeError {
            message: format!("Extension '{}' contains no CSS theme content", theme_id),
            span: None,
        })
    }
}
