//! Token definitions for the Justino programming language.

use crate::span::Span;

/// Enum describing all token types supported in `.jucode` source files.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // --- Keywords ---
    Fn,
    Let,
    Mut,
    If,
    Else,
    While,
    For,
    In,
    Return,
    Struct,
    Enum,
    Async,
    Await,
    Spawn,
    Import,
    Export,
    Match,
    True,
    False,
    Null,
    Try,
    Catch,

    // --- Operators ---
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `=`
    Assign,
    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `<`
    Less,
    /// `>`
    Greater,
    /// `<=`
    LessEqual,
    /// `>=`
    GreaterEqual,
    /// `&&`
    And,
    /// `||`
    Or,
    /// `!`
    Not,
    /// `->`
    Arrow,
    /// `=>`
    FatArrow,
    /// `.`
    Dot,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// `;`
    Semicolon,

    // --- Delimiters ---
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,

    // --- Literals ---
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),

    // --- String Interpolation Tokens ---
    /// Token representing `${` inside an interpolated string
    DollarBrace,
    /// String segment preceding `${` or following `}`
    StringSegment(String),

    // --- Special ---
    Eof,
}

/// A scanned Token carrying both its kind and source location `Span`.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}
