//! Expression name resolution.

use crate::resolver::Resolver;
use kivc_diagnostics::KivError;
use kivc_hir::{HirExpr, HirExprKind};
use kivc_span::Span;

impl Resolver {
    /// Resolve expression in-place
    pub(crate) fn resolve_expr_inplace(&mut self, expr: &mut HirExpr) {
        match &mut expr.kind {
            HirExprKind::Literal(_) => {}

            HirExprKind::Var { var_id, name } => {
                self.resolve_var(var_id, name, &expr.span);
            }

            HirExprKind::Binary { lhs, rhs, .. } => {
                self.resolve_expr_inplace(lhs);
                self.resolve_expr_inplace(rhs);
            }

            HirExprKind::Call { func: name, args } => {
                self.resolve_call(name, args, &expr.span);
            }

            HirExprKind::Assign {
                var_id,
                target,
                value,
            } => {
                self.resolve_assign(var_id, target, value, &expr.span);
            }

            HirExprKind::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr_inplace(cond);
                self.resolve_block_inplace(then_branch);
                if let Some(else_b) = else_branch {
                    self.resolve_block_inplace(else_b);
                }
            }
        }
    }

    fn resolve_var(&mut self, var_id: &mut kivc_hir::VarId, name: &str, span: &Span) {
        if let Some(symbol) = self.scope_manager.lookup(name) {
            match symbol.kind {
                crate::scope::SymbolKind::Variable(id) | crate::scope::SymbolKind::Constant(id) => {
                    *var_id = id;
                }
                _ => {
                    self.diagnostics.add(KivError::syntax(
                        span,
                        format!("'{}' is not a variable", name),
                        "expected variable",
                        None,
                        kivc_diagnostics::error_code::E001_UNEXPECTED_TOKEN,
                    ));
                }
            }
        } else {
            self.diagnostics.add(KivError::syntax(
                span,
                format!("undefined variable '{}'", name),
                "not found in this scope",
                None,
                kivc_diagnostics::error_code::E001_UNEXPECTED_TOKEN,
            ));
        }
    }

    fn resolve_call(&mut self, name: &str, args: &mut [HirExpr], span: &Span) {
        if !self.function_names.contains_key(name) {
            self.diagnostics.add(KivError::syntax(
                span,
                format!("undefined function '{}'", name),
                "not found",
                None,
                kivc_diagnostics::error_code::E001_UNEXPECTED_TOKEN,
            ));
        }
        for arg in args {
            self.resolve_expr_inplace(arg);
        }
    }

    fn resolve_assign(
        &mut self,
        var_id: &mut kivc_hir::VarId,
        target: &str,
        value: &mut Box<HirExpr>,
        span: &Span,
    ) {
        if let Some(symbol) = self.scope_manager.lookup(target) {
            match symbol.kind {
                crate::scope::SymbolKind::Variable(id) => {
                    *var_id = id;
                    if !symbol.is_mutable {
                        self.diagnostics.add(KivError::syntax(
                            span,
                            format!("cannot assign to immutable variable '{}'", target),
                            "not mutable",
                            Some("consider making it mutable with 'mut'".to_string()),
                            kivc_diagnostics::error_code::E001_UNEXPECTED_TOKEN,
                        ));
                    }
                }
                crate::scope::SymbolKind::Constant(_) => {
                    self.diagnostics.add(KivError::syntax(
                        span,
                        format!("cannot assign to constant '{}'", target),
                        "constants are immutable",
                        None,
                        kivc_diagnostics::error_code::E001_UNEXPECTED_TOKEN,
                    ));
                }
                _ => {
                    self.diagnostics.add(KivError::syntax(
                        span,
                        format!("'{}' is not a variable", target),
                        "expected variable",
                        None,
                        kivc_diagnostics::error_code::E001_UNEXPECTED_TOKEN,
                    ));
                }
            }
        } else {
            self.diagnostics.add(KivError::syntax(
                span,
                format!("undefined variable '{}'", target),
                "not found in this scope",
                None,
                kivc_diagnostics::error_code::E001_UNEXPECTED_TOKEN,
            ));
        }
        self.resolve_expr_inplace(value);
    }
}
