//! Statement name resolution.

use crate::resolver::Resolver;
use kivc_hir::{HirStmt, HirStmtKind};

impl Resolver {
    /// Resolve statement in-place
    pub(crate) fn resolve_stmt_inplace(&mut self, stmt: &mut HirStmt) {
        match &mut stmt.kind {
            HirStmtKind::Let {
                name,
                var_id,
                mutable,
                init,
                ..
            } => {
                self.resolve_expr_inplace(init);
                let symbol = crate::scope::Symbol::variable(
                    name.clone(),
                    *var_id,
                    *mutable,
                    stmt.span.clone(),
                );
                if let Err((_msg, original_span)) = self.scope_manager.insert(symbol) {
                    // Create multi-span error showing both declarations
                    let error = kivc_diagnostics::KivError::multi_span(
                        &stmt.span,
                        format!("variable '{}' is already declared in this scope", name),
                        "redeclared here",
                        vec![(&original_span, "originally declared here".to_string())],
                        Some("use a different name or remove one of the declarations".to_string()),
                        kivc_diagnostics::error_code::E001_UNEXPECTED_TOKEN,
                    );
                    self.diagnostics.add(error);
                }
            }

            HirStmtKind::Const {
                name,
                var_id,
                value,
                ..
            } => {
                self.resolve_expr_inplace(value);
                let symbol = crate::scope::Symbol::constant(name.clone(), *var_id, stmt.span.clone());
                if let Err((_msg, original_span)) = self.scope_manager.insert(symbol) {
                    // Create multi-span error showing both declarations
                    let error = kivc_diagnostics::KivError::multi_span(
                        &stmt.span,
                        format!("constant '{}' is already declared in this scope", name),
                        "redeclared here",
                        vec![(&original_span, "originally declared here".to_string())],
                        Some("use a different name or remove one of the declarations".to_string()),
                        kivc_diagnostics::error_code::E001_UNEXPECTED_TOKEN,
                    );
                    self.diagnostics.add(error);
                }
            }

            HirStmtKind::Return { value } => {
                if let Some(expr) = value {
                    self.resolve_expr_inplace(expr);
                }
            }

            HirStmtKind::Expr { expr } => {
                self.resolve_expr_inplace(expr);
            }
        }
    }
}
