//! Operator lexing tests.

mod common;
use kivc_lexer::TokenKind;

#[test]
fn test_operators() {
    let tokens = common::lex_source("= == != < <= > >= + - * /");
    assert_eq!(tokens.len(), 11);
    assert!(matches!(tokens[0].kind, TokenKind::Eq));
    assert!(matches!(tokens[1].kind, TokenKind::EqEq));
    assert!(matches!(tokens[2].kind, TokenKind::NotEq));
    assert!(matches!(tokens[3].kind, TokenKind::Lt));
    assert!(matches!(tokens[4].kind, TokenKind::Le));
    assert!(matches!(tokens[5].kind, TokenKind::Gt));
    assert!(matches!(tokens[6].kind, TokenKind::Ge));
    assert!(matches!(tokens[7].kind, TokenKind::Plus));
    assert!(matches!(tokens[8].kind, TokenKind::Minus));
    assert!(matches!(tokens[9].kind, TokenKind::Star));
    assert!(matches!(tokens[10].kind, TokenKind::Slash));
}

#[test]
fn test_delimiters() {
    let tokens = common::lex_source("(){}[];");
    assert_eq!(tokens.len(), 7);
    assert!(matches!(tokens[0].kind, TokenKind::LParen));
    assert!(matches!(tokens[1].kind, TokenKind::RParen));
    assert!(matches!(tokens[2].kind, TokenKind::LBrace));
    assert!(matches!(tokens[3].kind, TokenKind::RBrace));
    assert!(matches!(tokens[4].kind, TokenKind::LBracket));
    assert!(matches!(tokens[5].kind, TokenKind::RBracket));
    assert!(matches!(tokens[6].kind, TokenKind::Semicolon));
}
