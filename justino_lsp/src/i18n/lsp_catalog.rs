//! Multilingual Diagnostic Message Catalog for LSP (i18n).

use std::collections::HashMap;

pub struct LspCatalog {
    pub active_locale: String,
    pub messages: HashMap<String, HashMap<String, String>>,
}

impl LspCatalog {
    pub fn new(locale: &str) -> Self {
        let mut catalog = Self {
            active_locale: locale.to_string(),
            messages: HashMap::new(),
        };
        catalog.init_defaults();
        catalog
    }

    pub fn set_locale(&mut self, locale: &str) {
        self.active_locale = locale.to_string();
    }

    fn init_defaults(&mut self) {
        // pt-BR
        let mut pt = HashMap::new();
        pt.insert(
            "syntax_error_closing_brace".to_string(),
            "Erro de Sintaxe [Linha {line}, Coluna {col}]: Esperado '}}' para fechar o bloco da função '{name}'.".to_string(),
        );
        pt.insert(
            "unexpected_token".to_string(),
            "Erro de Sintaxe [Linha {line}, Coluna {col}]: Token inesperado '{token}'.".to_string(),
        );
        pt.insert(
            "unknown_identifier".to_string(),
            "Erro de Tipo [Linha {line}, Coluna {col}]: Identificador não encontrado '{name}'.".to_string(),
        );
        self.messages.insert("pt-BR".to_string(), pt);

        // en-US
        let mut en = HashMap::new();
        en.insert(
            "syntax_error_closing_brace".to_string(),
            "Syntax Error [Line {line}, Column {col}]: Expected '}}' to close function block '{name}'.".to_string(),
        );
        en.insert(
            "unexpected_token".to_string(),
            "Syntax Error [Line {line}, Column {col}]: Unexpected token '{token}'.".to_string(),
        );
        en.insert(
            "unknown_identifier".to_string(),
            "Type Error [Line {line}, Column {col}]: Unknown identifier '{name}'.".to_string(),
        );
        self.messages.insert("en-US".to_string(), en);

        // es-ES
        let mut es = HashMap::new();
        es.insert(
            "syntax_error_closing_brace".to_string(),
            "Error de Sintaxis [Línea {line}, Columna {col}]: Se esperaba '}}' para cerrar el bloque de la función '{name}'.".to_string(),
        );
        es.insert(
            "unexpected_token".to_string(),
            "Error de Sintaxis [Línea {line}, Columna {col}]: Token inesperado '{token}'.".to_string(),
        );
        es.insert(
            "unknown_identifier".to_string(),
            "Error de Tipo [Línea {line}, Columna {col}]: Identificador no encontrado '{name}'.".to_string(),
        );
        self.messages.insert("es-ES".to_string(), es);

        // zh-CN
        let mut zh = HashMap::new();
        zh.insert(
            "syntax_error_closing_brace".to_string(),
            "语法错误 [第 {line} 行，第 {col} 列]: 期望 '}}' 以结束函数块 '{name}'。".to_string(),
        );
        zh.insert(
            "unexpected_token".to_string(),
            "语法错误 [第 {line} 行，第 {col} 列]: 意外的标记 '{token}'。".to_string(),
        );
        zh.insert(
            "unknown_identifier".to_string(),
            "类型错误 [第 {line} 行，第 {col} 列]: 未知的标识符 '{name}'。".to_string(),
        );
        self.messages.insert("zh-CN".to_string(), zh);
    }

    pub fn format(&self, key: &str, line: u32, col: u32, name: &str, token: &str) -> String {
        let locale_map = self
            .messages
            .get(&self.active_locale)
            .or_else(|| self.messages.get("en-US"));

        let template = if let Some(map) = locale_map {
            map.get(key).cloned().unwrap_or_else(|| key.to_string())
        } else {
            key.to_string()
        };

        template
            .replace("{line}", &line.to_string())
            .replace("{col}", &col.to_string())
            .replace("{name}", name)
            .replace("{token}", token)
    }
}
