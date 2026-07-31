//! UTF-8 Scanner for the Justino programming language (.jucode).

use crate::error::JustinoError;
use crate::lexer::token::{Token, TokenKind};
use crate::span::Span;

/// High-performance UTF-8 Scanner.
pub struct Scanner<'a> {
    source: &'a str,
    file_id: usize,
    chars: Vec<(usize, char)>,
    cursor: usize,
    line: usize,
    column: usize,
    /// Stack tracking string interpolation depth
    string_depth_stack: Vec<usize>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str, file_id: usize) -> Self {
        let chars: Vec<(usize, char)> = source.char_indices().collect();
        Self {
            source,
            file_id,
            chars,
            cursor: 0,
            line: 1,
            column: 1,
            string_depth_stack: Vec::new(),
        }
    }

    /// Scans the source string and returns a vector of tokens or a LexError.
    pub fn scan(&mut self) -> Result<Vec<Token>, JustinoError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace_and_comments()?;
            if self.is_at_end() {
                break;
            }

            let start_byte = self.current_byte_offset();
            let start_line = self.line;
            let start_col = self.column;

            let (_idx, ch) = self.peek_char_info().ok_or_else(|| JustinoError::LexError {
                message: "Unexpected end of input".to_string(),
                span: Span::new(self.file_id, start_byte, start_byte, start_line, start_col),
            })?;

            // If we hit '}' and string_depth_stack is active, we might be closing an interpolation expression
            if ch == '}' && !self.string_depth_stack.is_empty() {
                let current_depth = self.string_depth_stack.last().copied().unwrap_or(0);
                if current_depth > 0 {
                    let depth_mut = self.string_depth_stack.last_mut().ok_or_else(|| JustinoError::LexError {
                        message: "Invalid string interpolation state".to_string(),
                        span: Span::new(self.file_id, start_byte, start_byte + 1, start_line, start_col),
                    })?;
                    *depth_mut -= 1;
                    if *depth_mut == 0 {
                        self.advance(); // consume '}'
                        let span = Span::new(self.file_id, start_byte, self.current_byte_offset(), start_line, start_col);
                        tokens.push(Token::new(TokenKind::RightBrace, span));
                        
                        // Resume scanning string body until next interpolation or closing quote
                        let str_tokens = self.scan_string_body(start_line, start_col)?;
                        tokens.extend(str_tokens);
                        continue;
                    }
                }
            }

            // Normal token scanning
            if is_identifier_start(ch) {
                let tok = self.scan_identifier_or_keyword(start_byte, start_line, start_col);
                tokens.push(tok);
            } else if ch.is_ascii_digit() {
                let tok = self.scan_number(start_byte, start_line, start_col)?;
                tokens.push(tok);
            } else if ch == '"' {
                let str_toks = self.scan_string_start(start_line, start_col)?;
                tokens.extend(str_toks);
            } else {
                let tok = self.scan_punctuation_or_operator(start_byte, start_line, start_col)?;
                tokens.push(tok);
            }
        }

        let end_byte = self.source.len();
        let eof_span = Span::new(self.file_id, end_byte, end_byte, self.line, self.column);
        tokens.push(Token::new(TokenKind::Eof, eof_span));

        Ok(tokens)
    }

    fn scan_identifier_or_keyword(&mut self, start_byte: usize, start_line: usize, start_col: usize) -> Token {
        let mut ident_str = String::new();
        while let Some((_, ch)) = self.peek_char_info() {
            if is_identifier_continue(ch) {
                ident_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let end_byte = self.current_byte_offset();
        let span = Span::new(self.file_id, start_byte, end_byte, start_line, start_col);

        let kind = match ident_str.as_str() {
            "fn" => TokenKind::Fn,
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "return" => TokenKind::Return,
            "struct" => TokenKind::Struct,
            "enum" => TokenKind::Enum,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "spawn" => TokenKind::Spawn,
            "import" => TokenKind::Import,
            "export" => TokenKind::Export,
            "match" => TokenKind::Match,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            "try" => TokenKind::Try,
            "catch" => TokenKind::Catch,
            _ => TokenKind::Identifier(ident_str),
        };

        Token::new(kind, span)
    }

    fn scan_number(&mut self, start_byte: usize, start_line: usize, start_col: usize) -> Result<Token, JustinoError> {
        let mut num_str = String::new();
        let mut is_float = false;

        while let Some((_, ch)) = self.peek_char_info() {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                // Peek next to ensure it's not a method call like `42.to_string()` or field access
                if let Some((_, next_ch)) = self.peek_next_char_info() {
                    if next_ch.is_ascii_digit() {
                        is_float = true;
                        num_str.push('.');
                        self.advance();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let end_byte = self.current_byte_offset();
        let span = Span::new(self.file_id, start_byte, end_byte, start_line, start_col);

        if is_float {
            let val = num_str.parse::<f64>().map_err(|_| JustinoError::LexError {
                message: format!("Invalid float literal '{}'", num_str),
                span,
            })?;
            Ok(Token::new(TokenKind::Float(val), span))
        } else {
            let val = num_str.parse::<i64>().map_err(|_| JustinoError::LexError {
                message: format!("Invalid integer literal '{}'", num_str),
                span,
            })?;
            Ok(Token::new(TokenKind::Int(val), span))
        }
    }

    fn scan_string_start(&mut self, start_line: usize, start_col: usize) -> Result<Vec<Token>, JustinoError> {
        // Consume opening '"'
        self.advance();
        self.scan_string_body(start_line, start_col)
    }

    fn scan_string_body(&mut self, start_line: usize, start_col: usize) -> Result<Vec<Token>, JustinoError> {
        let start_byte = self.current_byte_offset();
        let mut buf = String::new();
        let mut tokens = Vec::new();

        while let Some((_, ch)) = self.peek_char_info() {
            if ch == '"' {
                let end_byte = self.current_byte_offset();
                self.advance(); // consume closing '"'
                let span = Span::new(self.file_id, start_byte, end_byte + 1, start_line, start_col);
                
                if !self.string_depth_stack.is_empty() {
                    // String segment inside string interpolation sequence
                    tokens.push(Token::new(TokenKind::StringSegment(buf), span));
                    self.string_depth_stack.pop();
                } else {
                    // Normal full string
                    tokens.push(Token::new(TokenKind::String(buf), span));
                }
                return Ok(tokens);
            } else if ch == '$' {
                if let Some((_, '{')) = self.peek_next_char_info() {
                    let end_byte = self.current_byte_offset();
                    let span_segment = Span::new(self.file_id, start_byte, end_byte, start_line, start_col);
                    tokens.push(Token::new(TokenKind::StringSegment(buf.clone()), span_segment));

                    self.advance(); // consume '$'
                    self.advance(); // consume '{'

                    let interp_span = Span::new(self.file_id, end_byte, end_byte + 2, self.line, self.column);
                    tokens.push(Token::new(TokenKind::DollarBrace, interp_span));

                    // Push interpolation depth
                    if self.string_depth_stack.is_empty() {
                        self.string_depth_stack.push(1);
                    } else if let Some(depth) = self.string_depth_stack.last_mut() {
                        *depth += 1;
                    }

                    return Ok(tokens);
                } else {
                    buf.push('$');
                    self.advance();
                }
            } else if ch == '\\' {
                self.advance(); // consume '\\'
                if let Some((_, escaped)) = self.peek_char_info() {
                    match escaped {
                        'n' => buf.push('\n'),
                        'r' => buf.push('\r'),
                        't' => buf.push('\t'),
                        '"' => buf.push('"'),
                        '\\' => buf.push('\\'),
                        '0' => buf.push('\0'),
                        c => buf.push(c),
                    }
                    self.advance();
                } else {
                    return Err(JustinoError::LexError {
                        message: "Unterminated escape sequence in string".to_string(),
                        span: Span::new(self.file_id, start_byte, self.current_byte_offset(), start_line, start_col),
                    });
                }
            } else {
                buf.push(ch);
                self.advance();
            }
        }

        Err(JustinoError::LexError {
            message: "Unterminated string literal".to_string(),
            span: Span::new(self.file_id, start_byte, self.current_byte_offset(), start_line, start_col),
        })
    }

    fn scan_punctuation_or_operator(&mut self, start_byte: usize, start_line: usize, start_col: usize) -> Result<Token, JustinoError> {
        let (_, ch) = self.peek_char_info().ok_or_else(|| JustinoError::LexError {
            message: "Unexpected end of input".to_string(),
            span: Span::new(self.file_id, start_byte, start_byte, start_line, start_col),
        })?;

        self.advance();

        let (kind, len) = match ch {
            '+' => (TokenKind::Plus, 1),
            '-' => {
                if self.match_char('>') {
                    (TokenKind::Arrow, 2)
                } else {
                    (TokenKind::Minus, 1)
                }
            }
            '*' => (TokenKind::Star, 1),
            '/' => (TokenKind::Slash, 1),
            '%' => (TokenKind::Percent, 1),
            '=' => {
                if self.match_char('=') {
                    (TokenKind::Equal, 2)
                } else if self.match_char('>') {
                    (TokenKind::FatArrow, 2)
                } else {
                    (TokenKind::Assign, 1)
                }
            }
            '!' => {
                if self.match_char('=') {
                    (TokenKind::NotEqual, 2)
                } else {
                    (TokenKind::Not, 1)
                }
            }
            '<' => {
                if self.match_char('=') {
                    (TokenKind::LessEqual, 2)
                } else {
                    (TokenKind::Less, 1)
                }
            }
            '>' => {
                if self.match_char('=') {
                    (TokenKind::GreaterEqual, 2)
                } else {
                    (TokenKind::Greater, 1)
                }
            }
            '&' => {
                if self.match_char('&') {
                    (TokenKind::And, 2)
                } else {
                    return Err(JustinoError::LexError {
                        message: "Unexpected character '&', expected '&&'".to_string(),
                        span: Span::new(self.file_id, start_byte, start_byte + 1, start_line, start_col),
                    });
                }
            }
            '|' => {
                if self.match_char('|') {
                    (TokenKind::Or, 2)
                } else {
                    return Err(JustinoError::LexError {
                        message: "Unexpected character '|', expected '||'".to_string(),
                        span: Span::new(self.file_id, start_byte, start_byte + 1, start_line, start_col),
                    });
                }
            }
            '.' => (TokenKind::Dot, 1),
            ':' => (TokenKind::Colon, 1),
            ',' => (TokenKind::Comma, 1),
            ';' => (TokenKind::Semicolon, 1),
            '(' => (TokenKind::LeftParen, 1),
            ')' => (TokenKind::RightParen, 1),
            '{' => (TokenKind::LeftBrace, 1),
            '}' => (TokenKind::RightBrace, 1),
            '[' => (TokenKind::LeftBracket, 1),
            ']' => (TokenKind::RightBracket, 1),
            other => {
                return Err(JustinoError::LexError {
                    message: format!("Unexpected character '{}'", other),
                    span: Span::new(self.file_id, start_byte, start_byte + other.len_utf8(), start_line, start_col),
                });
            }
        };

        let span = Span::new(self.file_id, start_byte, start_byte + len, start_line, start_col);
        Ok(Token::new(kind, span))
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<(), JustinoError> {
        while let Some((start_byte, ch)) = self.peek_char_info() {
            match ch {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                }
                '/' => {
                    if let Some((_, '/')) = self.peek_next_char_info() {
                        // Single line comment: // ...
                        self.advance(); // consume '/'
                        self.advance(); // consume '/'
                        while let Some((_, c)) = self.peek_char_info() {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else if let Some((_, '*')) = self.peek_next_char_info() {
                        // Block comment: /* ... */
                        let start_line = self.line;
                        let start_col = self.column;
                        self.advance(); // consume '/'
                        self.advance(); // consume '*'

                        let mut closed = false;
                        while let Some((_, c)) = self.peek_char_info() {
                            if c == '*' {
                                self.advance();
                                if let Some((_, '/')) = self.peek_char_info() {
                                    self.advance();
                                    closed = true;
                                    break;
                                }
                            } else {
                                self.advance();
                            }
                        }

                        if !closed {
                            return Err(JustinoError::LexError {
                                message: "Unterminated block comment".to_string(),
                                span: Span::new(self.file_id, start_byte, self.current_byte_offset(), start_line, start_col),
                            });
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn peek_char_info(&self) -> Option<(usize, char)> {
        self.chars.get(self.cursor).copied()
    }

    fn peek_next_char_info(&self) -> Option<(usize, char)> {
        self.chars.get(self.cursor + 1).copied()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if let Some((_, ch)) = self.peek_char_info() {
            if ch == expected {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) {
        if let Some((_, ch)) = self.peek_char_info() {
            self.cursor += 1;
            if ch == '\n' {
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

    fn current_byte_offset(&self) -> usize {
        self.chars
            .get(self.cursor)
            .map(|(idx, _)| *idx)
            .unwrap_or(self.source.len())
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_' || (!ch.is_ascii() && ch.is_alphanumeric())
}

fn is_identifier_continue(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || (!ch.is_ascii() && ch.is_alphanumeric())
}
