//! CSS3 Parser and Rule Extractor.

use crate::css::stylesheet::*;
use crate::css::value::*;
use crate::error::UiError;
use std::collections::HashMap;

/// Parses raw CSS text into a structured `Stylesheet`.
pub struct CssParser<'a> {
    pub source: &'a str,
    chars: Vec<(usize, char)>,
    cursor: usize,
    line: usize,
    column: usize,
}

impl<'a> CssParser<'a> {
    pub fn new(source: &'a str) -> Self {
        let chars: Vec<(usize, char)> = source.char_indices().collect();
        Self {
            source,
            chars,
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    /// Parses the entire CSS input string.
    pub fn parse(&mut self) -> Result<Stylesheet, UiError> {
        let mut rules = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace_and_comments();
            if self.is_at_end() {
                break;
            }

            let rule = self.parse_rule()?;
            rules.push(rule);
        }

        Ok(Stylesheet { rules })
    }

    fn parse_rule(&mut self) -> Result<Rule, UiError> {
        let mut selectors = Vec::new();

        loop {
            let selector = self.parse_single_selector()?;
            selectors.push(selector);

            self.skip_whitespace_and_comments();
            if self.match_char(',') {
                self.skip_whitespace_and_comments();
            } else if self.check_char('{') {
                break;
            } else if self.is_at_end() {
                return Err(UiError::CssParseError {
                    message: "Unexpected EOF while reading selectors".to_string(),
                    line: self.line,
                    column: self.column,
                });
            }
        }

        self.expect_char('{')?;
        let declarations = self.parse_declarations()?;
        self.expect_char('}')?;

        Ok(Rule {
            selectors,
            declarations,
        })
    }

    fn parse_single_selector(&mut self) -> Result<Selector, UiError> {
        let mut tag = None;
        let mut id = None;
        let mut classes = Vec::new();
        let mut pseudo = None;

        while !self.is_at_end() {
            self.skip_whitespace_and_comments();
            let ch = match self.peek_char() {
                Some(c) => c,
                None => break,
            };

            if ch == '{' || ch == ',' {
                break;
            } else if ch == '#' {
                self.advance();
                let name = self.parse_identifier()?;
                id = Some(name);
            } else if ch == '.' {
                self.advance();
                let name = self.parse_identifier()?;
                classes.push(name);
            } else if ch == ':' {
                self.advance();
                let name = self.parse_identifier()?;
                if name == "lang" && self.match_char('(') {
                    let lang_val = self.parse_identifier()?;
                    self.match_char(')');
                    pseudo = Some(PseudoClass::Lang(lang_val));
                } else {
                    let p = match name.as_str() {
                        "hover" => PseudoClass::Hover,
                        "active" => PseudoClass::Active,
                        "focus" => PseudoClass::Focus,
                        _ => PseudoClass::Hover,
                    };
                    pseudo = Some(p);
                }
            } else if ch.is_alphanumeric() || ch == '_' || ch == '*' {
                let name = self.parse_identifier()?;
                tag = Some(name);
            } else {
                break;
            }
        }

        let spec = SelectorSpecificity::calculate(&tag, &id, &classes, &pseudo);

        Ok(Selector {
            tag,
            id,
            classes,
            pseudo,
            specificity: spec,
        })
    }

    fn parse_declarations(&mut self) -> Result<HashMap<String, CssValue>, UiError> {
        let mut map = HashMap::new();

        while !self.is_at_end() {
            self.skip_whitespace_and_comments();
            if self.check_char('}') {
                break;
            }

            let prop_name = self.parse_identifier()?;
            self.expect_char(':')?;
            self.skip_whitespace_and_comments();
            let prop_val = self.parse_value()?;
            
            map.insert(prop_name, prop_val);

            self.skip_whitespace_and_comments();
            if self.match_char(';') {
                self.skip_whitespace_and_comments();
            }
        }

        Ok(map)
    }

    fn parse_value(&mut self) -> Result<CssValue, UiError> {
        self.skip_whitespace_and_comments();
        let ch = self.peek_char().ok_or_else(|| UiError::CssParseError {
            message: "Unexpected end of input while reading property value".to_string(),
            line: self.line,
            column: self.column,
        })?;

        if ch == '#' {
            self.advance();
            let hex_str = self.parse_hex_code()?;
            let color = parse_hex_color(&hex_str)?;
            Ok(CssValue::Color(color))
        } else if ch.is_ascii_digit() || ch == '-' || ch == '.' {
            let (num, unit) = self.parse_number_with_unit()?;
            let val = match unit.as_str() {
                "px" => CssValue::Px(num),
                "rem" => CssValue::Rem(num),
                "em" => CssValue::Em(num),
                "%" => CssValue::Percent(num),
                "vh" => CssValue::Vh(num),
                "vw" => CssValue::Vw(num),
                _ => CssValue::Number(num),
            };
            Ok(val)
        } else {
            let ident = self.parse_identifier()?;
            let val = match ident.as_str() {
                "flex" => CssValue::Display(DisplayValue::Flex),
                "block" => CssValue::Display(DisplayValue::Block),
                "none" => CssValue::Display(DisplayValue::None),
                "row" => CssValue::Direction(FlexDirection::Row),
                "column" => CssValue::Direction(FlexDirection::Column),
                "row-reverse" => CssValue::Direction(FlexDirection::RowReverse),
                "column-reverse" => CssValue::Direction(FlexDirection::ColumnReverse),
                "auto" => CssValue::Auto,
                "transparent" => CssValue::Color(Color::TRANSPARENT),
                "black" => CssValue::Color(Color::BLACK),
                "white" => CssValue::Color(Color::WHITE),
                other => CssValue::Keyword(other.to_string()),
            };
            Ok(val)
        }
    }

    fn parse_identifier(&mut self) -> Result<String, UiError> {
        let mut buf = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() || ch == '-' || ch == '_' || ch == '*' {
                buf.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if buf.is_empty() {
            return Err(UiError::CssParseError {
                message: "Expected identifier".to_string(),
                line: self.line,
                column: self.column,
            });
        }
        Ok(buf)
    }

    fn parse_hex_code(&mut self) -> Result<String, UiError> {
        let mut buf = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_hexdigit() {
                buf.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Ok(buf)
    }

    fn parse_number_with_unit(&mut self) -> Result<(f32, String), UiError> {
        let mut num_str = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_digit() || ch == '.' || ch == '-' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let num: f32 = num_str.parse().map_err(|_| UiError::CssParseError {
            message: format!("Invalid CSS number '{}'", num_str),
            line: self.line,
            column: self.column,
        })?;

        let mut unit = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_alphabetic() || ch == '%' {
                unit.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Ok((num, unit))
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' {
                if let Some('*') = self.peek_next_char() {
                    self.advance(); // consume '/'
                    self.advance(); // consume '*'
                    while let Some(c) = self.peek_char() {
                        if c == '*' {
                            self.advance();
                            if let Some('/') = self.peek_char() {
                                self.advance();
                                break;
                            }
                        } else {
                            self.advance();
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.cursor).map(|(_, c)| *c)
    }

    fn peek_next_char(&self) -> Option<char> {
        self.chars.get(self.cursor + 1).map(|(_, c)| *c)
    }

    fn check_char(&self, expected: char) -> bool {
        self.peek_char() == Some(expected)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.check_char(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_char(&mut self, expected: char) -> Result<(), UiError> {
        if self.match_char(expected) {
            Ok(())
        } else {
            Err(UiError::CssParseError {
                message: format!("Expected character '{}', found '{:?}'", expected, self.peek_char()),
                line: self.line,
                column: self.column,
            })
        }
    }

    fn advance(&mut self) {
        if let Some((_, ch)) = self.chars.get(self.cursor) {
            self.cursor += 1;
            if *ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.chars.len()
    }
}

fn parse_hex_color(hex: &str) -> Result<Color, UiError> {
    let hex = hex.trim_start_matches('#');
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(0);
            Ok(Color::rgb(r, g, b))
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            Ok(Color::rgb(r, g, b))
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            Ok(Color::rgba(r, g, b, a))
        }
        _ => Ok(Color::BLACK),
    }
}
