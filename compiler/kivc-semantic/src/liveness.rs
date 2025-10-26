//! Liveness analysis.
//!
//! Detects unused variables and other liveness issues.

use kivc_diagnostics::DiagnosticsCollector;
use kivc_hir::{HirBlock, HirExpr, HirExprKind, HirFunDef, HirStmt, HirStmtKind, VarId};
use std::collections::{HashMap, HashSet};

/// Analyzes variable liveness
pub struct LivenessAnalyzer {
    pub diagnostics: DiagnosticsCollector,
    used_vars: HashSet<VarId>,
    defined_vars: HashMap<VarId, String>,
}

impl LivenessAnalyzer {
    pub fn new() -> Self {
        Self {
            diagnostics: DiagnosticsCollector::new(),
            used_vars: HashSet::new(),
            defined_vars: HashMap::new(),
        }
    }

    pub fn analyze_function(&mut self, function: &HirFunDef) {
        // Register parameters as used (they're part of the function signature)
        for param in &function.params {
            self.defined_vars.insert(param.var_id, param.name.clone());
            self.used_vars.insert(param.var_id);
        }

        self.analyze_block(&function.body);

        // Report unused variables
        for (var_id, name) in &self.defined_vars {
            if !self.used_vars.contains(var_id) {
                self.diagnostics
                    .warning(format!("Unused variable '{}'", name));
            }
        }
    }

    fn analyze_block(&mut self, block: &HirBlock) {
        for stmt in &block.stmts {
            self.analyze_stmt(stmt);
        }
    }

    fn analyze_stmt(&mut self, stmt: &HirStmt) {
        match &stmt.kind {
            HirStmtKind::Let {
                name, var_id, init, ..
            } => {
                self.defined_vars.insert(*var_id, name.clone());
                self.analyze_expr(init);
            }

            HirStmtKind::Const {
                name,
                var_id,
                value,
                ..
            } => {
                self.defined_vars.insert(*var_id, name.clone());
                self.used_vars.insert(*var_id); // Constants are always considered "used"
                self.analyze_expr(value);
            }

            HirStmtKind::Return { value } => {
                if let Some(expr) = value {
                    self.analyze_expr(expr);
                }
            }

            HirStmtKind::Expr { expr } => {
                self.analyze_expr(expr);
            }
        }
    }

    fn analyze_expr(&mut self, expr: &HirExpr) {
        match &expr.kind {
            HirExprKind::Literal(_) => {}

            HirExprKind::Var { var_id, .. } => {
                self.used_vars.insert(*var_id);
            }

            HirExprKind::Binary { lhs, rhs, .. } => {
                self.analyze_expr(lhs);
                self.analyze_expr(rhs);
            }

            HirExprKind::Call { args, .. } => {
                for arg in args {
                    self.analyze_expr(arg);
                }
            }

            HirExprKind::Assign { var_id, value, .. } => {
                self.used_vars.insert(*var_id);
                self.analyze_expr(value);
            }

            HirExprKind::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.analyze_expr(cond);
                self.analyze_block(then_branch);

                if let Some(else_b) = else_branch {
                    self.analyze_block(else_b);
                }
            }
        }
    }
}

impl Default for LivenessAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
