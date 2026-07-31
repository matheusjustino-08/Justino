//! Locale and Translation Dictionary Manager.

use std::collections::HashMap;

pub struct LocaleManager {
    pub active_locale: String,
    pub translations: HashMap<String, HashMap<String, String>>,
}

impl LocaleManager {
    pub fn new(initial_locale: &str) -> Self {
        Self {
            active_locale: initial_locale.to_string(),
            translations: HashMap::new(),
        }
    }

    pub fn set_locale(&mut self, locale_code: &str) {
        self.active_locale = locale_code.to_string();
    }

    pub fn register_translation(&mut self, locale_code: &str, key: &str, value: &str) {
        self.translations
            .entry(locale_code.to_string())
            .or_insert_with(HashMap::new)
            .insert(key.to_string(), value.to_string());
    }

    pub fn translate(&self, key: &str) -> String {
        if let Some(locale_map) = self.translations.get(&self.active_locale) {
            if let Some(val) = locale_map.get(key) {
                return val.clone();
            }
        }
        key.to_string()
    }
}
