//! Expression type checking.

use crate::checker::TypeChecker;
use crate::types::Type;
use kivc_diagnostics::KivError;
use kivc_hir::{HirExpr, HirExprKind};

/// Check an expression in-place
pub fn check_expr_inplace(checker: &mut TypeChecker, expr: &mut HirExpr) {
    match &mut expr.kind {
        HirExprKind::Literal(_) => {}

        HirExprKind::Var { var_id, name } => {
            if !checker.var_types.contains_key(var_id) {
                checker.diagnostics.add(KivError::syntax(
                    &expr.span,
                    format!("undefined variable '{}'", name),
                    "not found",
                    None,
                    kivc_diagnostics::error_code::E100_TYPE_MISMATCH,
                ));
            }
        }

        HirExprKind::Binary { op, lhs, rhs } => {
            checker.check_expr_inplace(lhs);
            checker.check_expr_inplace(rhs);

            let lhs_type = checker.infer_expr_type(lhs);
            let rhs_type = checker.infer_expr_type(rhs);

            if let Some(result_type) = checker.ctx.binary_op_result_type(*op, &lhs_type, &rhs_type)
            {
                expr.ty = checker.type_to_type_id(&result_type);
            } else {
                checker.diagnostics.add(KivError::type_error(
                    &expr.span,
                    format!(
                        "cannot apply operator {:?} to {} and {}",
                        op,
                        lhs_type.name(),
                        rhs_type.name()
                    ),
                    "incompatible types for operation",
                    None,
                    kivc_diagnostics::error_code::E100_TYPE_MISMATCH,
                ));
            }
        }

        HirExprKind::Call { args, .. } => {
            for arg in args {
                checker.check_expr_inplace(arg);
            }
        }

        HirExprKind::Assign {
            var_id,
            target,
            value,
        } => {
            checker.check_expr_inplace(value);

            if let Some(&mutable) = checker.var_mutability.get(var_id)
                && !mutable
            {
                checker.diagnostics.add(KivError::syntax(
                    &expr.span,
                    format!("cannot assign to immutable variable '{}'", target),
                    "not mutable",
                    Some("consider making it mutable with 'mut'".to_string()),
                    kivc_diagnostics::error_code::E100_TYPE_MISMATCH,
                ));
            }

            if let Some(var_type) = checker.var_types.get(var_id) {
                let value_type = checker.infer_expr_type(value);
                if !checker.ctx.are_compatible(var_type, &value_type) {
                    checker.diagnostics.add(KivError::type_error(
                        &value.span,
                        format!(
                            "type mismatch in assignment: expected {}, found {}",
                            var_type.name(),
                            value_type.name()
                        ),
                        "incompatible type",
                        None,
                        kivc_diagnostics::error_code::E100_TYPE_MISMATCH,
                    ));
                }
            }
        }

        HirExprKind::If {
            cond,
            then_branch,
            else_branch,
        } => {
            checker.check_expr_inplace(cond);

            let cond_type = checker.infer_expr_type(cond);
            if cond_type != Type::Bool && cond_type != Type::Unknown {
                checker.diagnostics.add(KivError::type_error(
                    &cond.span,
                    format!("if condition must be Bool, found {}", cond_type.name()),
                    "expected Bool",
                    None,
                    kivc_diagnostics::error_code::E100_TYPE_MISMATCH,
                ));
            }

            checker.check_block_inplace(then_branch);

            if let Some(else_b) = else_branch {
                checker.check_block_inplace(else_b);
            }
        }
    }
}
