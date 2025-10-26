//! Main compilation pipeline.

use crate::config::{CompilerConfig, StopAfter};
use crate::session::CompilerSession;
use kivc_ast::Program as AstProgram;
use kivc_codegen::generate_llvm_ir;
use kivc_diagnostics::DiagnosticsCollector;
use kivc_hir::{HirProgram, lower_program};
use kivc_mir::{MirProgram, lower_to_mir};
use kivc_parser::Parser;
use kivc_span::SourceFile;
use kivc_typeck::typecheck_program;
use std::sync::Arc;

/// Compilation output containing all compilation artifacts
#[derive(Debug)]
pub struct CompilerOutput {
    /// The diagnostics collector with all warnings/errors
    pub diagnostics: DiagnosticsCollector,

    /// AST if compilation reached this stage
    pub ast: Option<AstProgram>,

    /// HIR if compilation reached this stage
    pub hir: Option<HirProgram>,

    /// MIR if compilation reached this stage
    pub mir: Option<MirProgram>,

    /// LLVM IR if compilation completed
    pub llvm_ir: Option<String>,
}

impl CompilerOutput {
    pub fn new() -> Self {
        Self {
            diagnostics: DiagnosticsCollector::new(),
            ast: None,
            hir: None,
            mir: None,
            llvm_ir: None,
        }
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.has_errors()
    }
}

impl Default for CompilerOutput {
    fn default() -> Self {
        Self::new()
    }
}

/// Compile source code through the entire pipeline
pub fn compile_source(source: &str, filename: &str, config: &CompilerConfig) -> CompilerOutput {
    let mut output = CompilerOutput::new();
    let mut session = CompilerSession::new(config.clone());

    // Create source file
    let source_file = Arc::new(SourceFile::new(filename.to_string(), source.to_string()));

    // Stage 1: Lexing & Parsing -> AST
    let ast = match parse_source(&source_file, &mut session) {
        Ok(ast) => ast,
        Err(()) => {
            output.diagnostics = session.diagnostics;
            return output;
        }
    };

    output.ast = Some(ast.clone());
    if config.stop_after == Some(StopAfter::Parse) {
        output.diagnostics = session.diagnostics;
        return output;
    }

    // Stage 2: Lowering AST -> HIR
    let mut hir = match lower_program(ast) {
        Ok(hir) => hir,
        Err(diagnostics) => {
            output.diagnostics = diagnostics;
            return output;
        }
    };
    output.hir = Some(hir.clone());

    if config.stop_after == Some(StopAfter::Hir) {
        output.diagnostics = session.diagnostics;
        return output;
    }

    // Stage 3: Name Resolution (in-place)
    if kivc_resolve::resolve_names(&mut hir, &mut session.diagnostics).is_err() {
        output.hir = Some(hir);
        output.diagnostics = session.diagnostics;
        return output;
    }
    output.hir = Some(hir.clone());

    // Stage 4: Type Checking (in-place)
    if typecheck_program(&mut hir, &mut session.diagnostics).is_err() {
        output.hir = Some(hir);
        output.diagnostics = session.diagnostics;
        return output;
    }
    output.hir = Some(hir.clone());

    if config.stop_after == Some(StopAfter::TypeCheck) {
        output.diagnostics = session.diagnostics;
        return output;
    }

    // Stage 5: Semantic Analysis
    if kivc_semantic::analyze_semantics(&hir, &mut session.diagnostics).is_err() {
        output.diagnostics = session.diagnostics;
        return output;
    }

    // Stage 6: Lowering HIR -> MIR
    let mir = lower_to_mir(hir);
    output.mir = Some(mir.clone());

    if config.stop_after == Some(StopAfter::Mir) {
        output.diagnostics = session.diagnostics;
        return output;
    }

    // Stage 7: Code Generation (LLVM IR)
    match generate_llvm_ir(&mir) {
        Ok(llvm_ir) => {
            output.llvm_ir = Some(llvm_ir);
        }
        Err(e) => {
            session
                .diagnostics
                .error(format!("Code generation failed: {}", e));
        }
    }

    output.diagnostics = session.diagnostics;
    output
}

/// Parse source code into AST
fn parse_source(
    source_file: &Arc<SourceFile>,
    session: &mut CompilerSession,
) -> Result<AstProgram, ()> {
    let mut parser = Parser::new(source_file.clone());
    let ast = parser.parse_program();

    // Collect parser diagnostics
    session.diagnostics.merge(parser.diagnostics().clone());

    if session.diagnostics.has_errors() {
        Err(())
    } else {
        Ok(ast)
    }
}

/// Compile to AST only (for testing/debugging)
pub fn compile_to_ast(source: &str, filename: &str) -> Result<AstProgram, DiagnosticsCollector> {
    let config = CompilerConfig {
        stop_after: Some(StopAfter::Parse),
        ..Default::default()
    };

    let output = compile_source(source, filename, &config);

    if let Some(ref ast) = output.ast {
        if output.has_errors() {
            Err(output.diagnostics)
        } else {
            Ok(ast.clone())
        }
    } else {
        Err(output.diagnostics)
    }
}

/// Compile to HIR only (for testing/debugging)
pub fn compile_to_hir(source: &str, filename: &str) -> Result<HirProgram, DiagnosticsCollector> {
    let config = CompilerConfig {
        stop_after: Some(StopAfter::Hir),
        ..Default::default()
    };

    let output = compile_source(source, filename, &config);

    if let Some(ref hir) = output.hir {
        if output.has_errors() {
            Err(output.diagnostics)
        } else {
            Ok(hir.clone())
        }
    } else {
        Err(output.diagnostics)
    }
}

/// Compile to MIR only (for testing/debugging)
pub fn compile_to_mir(source: &str, filename: &str) -> Result<MirProgram, DiagnosticsCollector> {
    let config = CompilerConfig::default();

    let output = compile_source(source, filename, &config);

    if let Some(ref mir) = output.mir {
        if output.has_errors() {
            Err(output.diagnostics)
        } else {
            Ok(mir.clone())
        }
    } else {
        Err(output.diagnostics)
    }
}
