//! Statement type checking.

use crate::checker::TypeChecker;
use crate::types::Type;
use kivc_diagnostics::KivError;
use kivc_hir::{HirStmt, HirStmtKind};

/// Check a statement in-place
pub fn check_stmt_inplace(checker: &mut TypeChecker, stmt: &mut HirStmt) {
    match &mut stmt.kind {
        HirStmtKind::Let {
            var_id,
            mutable,
            ty,
            init,
            ..
        } => {
            checker.check_expr_inplace(init);
            let init_type = checker.infer_expr_type(init);

            let var_type = if let Some(ty_id) = ty {
                let expected_type = checker.ctx.get(*ty_id).cloned().unwrap_or(Type::Unknown);
                if !checker.ctx.are_compatible(&expected_type, &init_type) {
                    checker.diagnostics.add(KivError::type_error(
                        &init.span,
                        format!(
                            "type mismatch: expected {}, found {}",
                            expected_type.name(),
                            init_type.name()
                        ),
                        "incompatible type",
                        None,
                        kivc_diagnostics::error_code::E100_TYPE_MISMATCH,
                    ));
                }
                expected_type
            } else {
                init_type
            };

            checker.var_types.insert(*var_id, var_type);
            checker.var_mutability.insert(*var_id, *mutable);
        }

        HirStmtKind::Const { var_id, value, .. } => {
            checker.check_expr_inplace(value);
            let value_type = checker.infer_expr_type(value);
            checker.var_types.insert(*var_id, value_type);
            checker.var_mutability.insert(*var_id, false);
        }

        HirStmtKind::Return { value } => {
            if let Some(ret_expr) = value {
                checker.check_expr_inplace(ret_expr);
                let ret_type = checker.infer_expr_type(ret_expr);

                if let Some(expected_ty_id) = checker.current_return_type {
                    let expected_type = checker
                        .ctx
                        .get(expected_ty_id)
                        .cloned()
                        .unwrap_or(Type::Unknown);

                    if !checker.ctx.are_compatible(&expected_type, &ret_type) {
                        checker.diagnostics.add(KivError::type_error(
                            &ret_expr.span,
                            format!(
                                "return type mismatch: expected {}, found {}",
                                expected_type.name(),
                                ret_type.name()
                            ),
                            "incompatible return type",
                            None,
                            kivc_diagnostics::error_code::E100_TYPE_MISMATCH,
                        ));
                    }
                }
            }
        }

        HirStmtKind::Expr { expr } => {
            checker.check_expr_inplace(expr);
        }
    }
}
