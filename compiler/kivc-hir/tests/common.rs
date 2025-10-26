//! Common test utilities for HIR lower tests.

use kivc_hir::lower_program;
use kivc_parser::Parser;
use kivc_span::SourceFile;

pub fn parse_and_lower(source: &str) -> kivc_hir::HirProgram {
    let file = SourceFile::new("test.kiv".to_string(), source.to_string());
    let mut parser = Parser::new(file);
    let program = parser.parse_program();

    lower_program(program).expect("lowering should succeed")
}
