//! Common test utilities for MIR tests.

use kivc_hir::lower_program;
use kivc_mir::lower_to_mir;
use kivc_parser::Parser;
use kivc_span::SourceFile;
use kivc_typeck::typecheck_program;
use kivc_diagnostics::DiagnosticsCollector;

pub fn parse_lower_typecheck_and_mir(source: &str) -> kivc_mir::MirProgram {
    let file = SourceFile::new("test.kiv".to_string(), source.to_string());
    let mut parser = Parser::new(file);
    let program = parser.parse_program();

    let mut hir = lower_program(program).expect("lowering should succeed");
    let mut diagnostics = DiagnosticsCollector::new();
    typecheck_program(&mut hir, &mut diagnostics).expect("typechecking should succeed");

    lower_to_mir(hir)
}
