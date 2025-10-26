//! Type checking for the Kiv language.
//!
//! This crate performs type checking on HIR (High-level Intermediate Representation).

mod checker;
mod expr;
mod inference;
mod stmt;
pub mod types;

pub use checker::TypeChecker;
pub use types::{Type, TypeContext};

use kivc_diagnostics::{CompilationFailed, DiagnosticsCollector};
use kivc_hir::HirProgram;

/// Type checks a HIR program in-place
///
/// Modifies the program in-place and returns `Ok(())` on success.
/// Returns `Err(CompilationFailed)` if any type errors are encountered.
/// Detailed error information is collected in `diagnostics`.
pub fn typecheck_program(
    program: &mut HirProgram,
    diagnostics: &mut DiagnosticsCollector,
) -> Result<(), CompilationFailed> {
    let mut checker = TypeChecker::new();
    checker.check_program_inplace(program);

    // Merge diagnostics
    diagnostics.merge(checker.diagnostics);

    if diagnostics.has_errors() {
        Err(CompilationFailed)
    } else {
        Ok(())
    }
}
