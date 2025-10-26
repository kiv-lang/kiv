//! Common test utilities for type checking tests.

use kivc_hir::lower_program;
use kivc_parser::Parser;
use kivc_span::SourceFile;
use kivc_typeck::typecheck_program;
use kivc_diagnostics::DiagnosticsCollector;

pub fn parse_lower_and_typecheck(source: &str) -> Result<(), String> {
    let file = SourceFile::new("test.kiv".to_string(), source.to_string());
    let mut parser = Parser::new(file);
    let program = parser.parse_program();

    let mut hir = lower_program(program).map_err(|_| "lowering failed".to_string())?;
    let mut diagnostics = DiagnosticsCollector::new();

    typecheck_program(&mut hir, &mut diagnostics).map_err(|_| "type checking failed".to_string())?;

    Ok(())
}
