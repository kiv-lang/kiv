//! Semantic analysis for the Kiv language.
//!
//! This module performs semantic checks beyond type checking, including:
//! - Control flow analysis
//! - Reachability analysis
//! - Dead code detection
//! - Unused variable warnings

mod analyzer;
mod control_flow;
mod liveness;

pub use analyzer::SemanticAnalyzer;

use kivc_diagnostics::{CompilationFailed, DiagnosticsCollector};
use kivc_hir::HirProgram;

/// Perform semantic analysis on a HIR program
///
/// Returns `Err(CompilationFailed)` if any semantic errors are encountered.
/// Detailed error information is collected in `diagnostics`.
pub fn analyze_semantics(
    program: &HirProgram,
    diagnostics: &mut DiagnosticsCollector,
) -> Result<(), CompilationFailed> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze_program(program);

    // Merge diagnostics
    diagnostics.merge(analyzer.diagnostics);

    if diagnostics.has_errors() {
        Err(CompilationFailed)
    } else {
        Ok(())
    }
}
