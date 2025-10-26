//! Literal lexing tests.

mod common;
use kivc_lexer::TokenKind;

#[test]
fn test_identifiers() {
    let tokens = common::lex_source("foo bar _baz x123");
    assert_eq!(tokens.len(), 4);
    assert!(matches!(tokens[0].kind, TokenKind::Ident(_)));
    assert!(matches!(tokens[1].kind, TokenKind::Ident(_)));
    assert!(matches!(tokens[2].kind, TokenKind::Ident(_)));
    assert!(matches!(tokens[3].kind, TokenKind::Ident(_)));
}

#[test]
fn test_integers() {
    let tokens = common::lex_source("0 42 1000");
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, TokenKind::IntLit(0));
    assert_eq!(tokens[1].kind, TokenKind::IntLit(42));
    assert_eq!(tokens[2].kind, TokenKind::IntLit(1000));
}

#[test]
fn test_floats() {
    let tokens = common::lex_source("2.71 0.5 123.456");
    assert_eq!(tokens.len(), 3);
    assert!(matches!(tokens[0].kind, TokenKind::FloatLit(f) if (f - 2.71).abs() < 0.001));
    assert!(matches!(tokens[1].kind, TokenKind::FloatLit(f) if (f - 0.5).abs() < 0.001));
    assert!(matches!(tokens[2].kind, TokenKind::FloatLit(f) if (f - 123.456).abs() < 0.001));
}

#[test]
fn test_strings() {
    let tokens = common::lex_source(r#""hello" "world""#);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::TextLit("hello".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::TextLit("world".to_string()));
}

#[test]
fn test_string_escapes() {
    let tokens = common::lex_source(r#""hello\nworld" "tab\there""#);
    assert_eq!(tokens.len(), 2);
    assert_eq!(
        tokens[0].kind,
        TokenKind::TextLit("hello\nworld".to_string())
    );
    assert_eq!(tokens[1].kind, TokenKind::TextLit("tab\there".to_string()));
}
