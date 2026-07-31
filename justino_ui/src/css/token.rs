//! CSS3 Tokenizer definitions.

#[derive(Debug, Clone, PartialEq)]
pub enum CssTokenKind {
    Ident(String),
    DotIdent(String),     // .class
    HashIdent(String),    // #id
    ColonIdent(String),   // :hover, :focus, etc.
    Number(f32, String),  // (value, unit like "px", "%", "rem", "")
    ColorHex(String),     // #ffffff
    LeftBrace,            // {
    RightBrace,           // }
    Colon,                // :
    Semicolon,            // ;
    Comma,                // ,
    LeftParen,            // (
    RightParen,           // )
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CssToken {
    pub kind: CssTokenKind,
    pub line: usize,
    pub column: usize,
}

impl CssToken {
    pub fn new(kind: CssTokenKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}
