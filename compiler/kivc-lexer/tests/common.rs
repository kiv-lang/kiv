//! Common test utilities for lexer tests.

use kivc_lexer::{Lexer, TokenKind};
use kivc_span::SourceFile;

pub fn lex_source(source: &str) -> Vec<kivc_lexer::Token> {
    let file = SourceFile::new("test.kiv".to_string(), source.to_string());
    let mut lexer = Lexer::new(file);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        if token.kind == TokenKind::Eof {
            break;
        }
        tokens.push(token);
    }

    tokens
}
