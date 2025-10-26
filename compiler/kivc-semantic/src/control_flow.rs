//! Control flow analysis.
//!
//! Checks for unreachable code and ensures all code paths return appropriately.

use kivc_diagnostics::DiagnosticsCollector;
use kivc_hir::{HirBlock, HirExpr, HirExprKind, HirFunDef, HirStmt, HirStmtKind};

/// Analyzes control flow in functions
pub struct ControlFlowAnalyzer {
    pub diagnostics: DiagnosticsCollector,
}

impl ControlFlowAnalyzer {
    pub fn new() -> Self {
        Self {
            diagnostics: DiagnosticsCollector::new(),
        }
    }

    pub fn analyze_function(&mut self, function: &HirFunDef) {
        let returns = self.analyze_block(&function.body);

        // Check if function returns when it should
        if function.return_type.is_some() && !returns {
            self.diagnostics.warning(format!(
                "Function '{}' may not return a value in all code paths",
                function.name
            ));
        }
    }

    /// Analyze a block and return whether it always returns
    fn analyze_block(&mut self, block: &HirBlock) -> bool {
        let mut unreachable = false;

        for (i, stmt) in block.stmts.iter().enumerate() {
            if unreachable {
                self.diagnostics
                    .warning("Unreachable code after return statement".to_string());
                break;
            }

            if self.stmt_returns(stmt) {
                unreachable = true;

                // Check if there are more statements after this
                if i < block.stmts.len() - 1 {
                    self.diagnostics
                        .warning("Code after return statement is unreachable".to_string());
                }
            }
        }

        unreachable
    }

    /// Check if a statement always returns
    fn stmt_returns(&mut self, stmt: &HirStmt) -> bool {
        match &stmt.kind {
            HirStmtKind::Return { .. } => true,

            HirStmtKind::Expr { expr } => self.expr_returns(expr),

            _ => false,
        }
    }

    /// Check if an expression always returns
    fn expr_returns(&mut self, expr: &HirExpr) -> bool {
        match &expr.kind {
            HirExprKind::If {
                then_branch,
                else_branch,
                ..
            } => {
                let then_returns = self.analyze_block(then_branch);

                if let Some(else_b) = else_branch {
                    let else_returns = self.analyze_block(else_b);
                    then_returns && else_returns
                } else {
                    false
                }
            }

            _ => false,
        }
    }
}

impl Default for ControlFlowAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
