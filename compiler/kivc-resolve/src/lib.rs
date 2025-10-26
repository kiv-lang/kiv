//! Name resolution for the Kiv language.
//!
//! This module handles resolving names to their definitions, managing scopes,
//! and detecting name conflicts.

mod block;
mod expr;
mod function;
mod program;
mod resolver;
mod scope;
mod stmt;

pub use resolver::Resolver;
pub use scope::{Scope, ScopeId, Symbol, SymbolKind};

use kivc_diagnostics::{CompilationFailed, DiagnosticsCollector};
use kivc_hir::HirProgram;

/// Perform name resolution on a HIR program in-place
///
/// Returns `Err(CompilationFailed)` if any errors are encountered.
/// Detailed error information is collected in `diagnostics`.
pub fn resolve_names(
    program: &mut HirProgram,
    diagnostics: &mut DiagnosticsCollector,
) -> Result<(), CompilationFailed> {
    let mut resolver = Resolver::new();
    resolver.resolve_program_inplace(program);

    // Merge diagnostics
    diagnostics.merge(resolver.diagnostics);

    if diagnostics.has_errors() {
        Err(CompilationFailed)
    } else {
        Ok(())
    }
}
