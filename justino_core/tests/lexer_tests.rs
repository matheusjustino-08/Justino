use justino_core::lexer::{Scanner, TokenKind};
use justino_core::JustinoError;

#[test]
fn test_lexer_utf8_and_keywords() -> Result<(), JustinoError> {
    let code = r#"
        // Line comment in English
        let mut heart: int = 100;
        let unicode_str = "Unicode Success!";
        if heart > 50 {
            return true;
        } else {
            return false;
        }
    "#;

    let mut scanner = Scanner::new(code, 1);
    let tokens = scanner.scan()?;

    assert!(tokens.iter().any(|t| matches!(&t.kind, TokenKind::Let)));
    assert!(tokens.iter().any(|t| matches!(&t.kind, TokenKind::Mut)));
    assert!(tokens.iter().any(|t| match &t.kind {
        TokenKind::Identifier(name) => name == "heart",
        _ => false,
    }));
    assert!(tokens.iter().any(|t| match &t.kind {
        TokenKind::String(val) => val == "Unicode Success!",
        _ => false,
    }));

    Ok(())
}

#[test]
fn test_lexer_string_interpolation() -> Result<(), JustinoError> {
    let code = r#"let msg = "Hello, ${name}!";"#;

    let mut scanner = Scanner::new(code, 1);
    let tokens = scanner.scan()?;

    assert!(tokens.iter().any(|t| match &t.kind {
        TokenKind::StringSegment(s) => s == "Hello, ",
        _ => false,
    }));
    assert!(tokens.iter().any(|t| matches!(&t.kind, TokenKind::DollarBrace)));
    assert!(tokens.iter().any(|t| match &t.kind {
        TokenKind::Identifier(id) => id == "name",
        _ => false,
    }));

    Ok(())
}
