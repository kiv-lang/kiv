//! Keyword lexing tests.

mod common;

#[test]
fn test_keywords() {
    let tokens = common::lex_source("fun let mut const if else return");
    assert_eq!(tokens.len(), 7);
    use kivc_lexer::TokenKind;
    assert!(matches!(tokens[0].kind, TokenKind::Fun));
    assert!(matches!(tokens[1].kind, TokenKind::Let));
    assert!(matches!(tokens[2].kind, TokenKind::Mut));
    assert!(matches!(tokens[3].kind, TokenKind::Const));
    assert!(matches!(tokens[4].kind, TokenKind::If));
    assert!(matches!(tokens[5].kind, TokenKind::Else));
    assert!(matches!(tokens[6].kind, TokenKind::Return));
}
