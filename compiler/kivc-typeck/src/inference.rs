//! Type inference implementation.

use crate::checker::TypeChecker;
use crate::types::Type;
use kivc_hir::{HirExpr, HirExprKind, TypeId};

/// Infer the type of an expression
pub fn infer_expr_type(checker: &TypeChecker, expr: &HirExpr) -> Type {
    // If type is already set, use it
    if let Some(ty_id) = expr.ty {
        return checker.ctx.get(ty_id).cloned().unwrap_or(Type::Unknown);
    }

    match &expr.kind {
        HirExprKind::Literal(lit) => infer_literal_type(lit),

        HirExprKind::Var { var_id, .. } => checker
            .var_types
            .get(var_id)
            .cloned()
            .unwrap_or(Type::Unknown),

        HirExprKind::Binary { op, lhs, rhs } => {
            let lhs_type = infer_expr_type(checker, lhs);
            let rhs_type = infer_expr_type(checker, rhs);

            checker
                .ctx
                .binary_op_result_type(*op, &lhs_type, &rhs_type)
                .unwrap_or(Type::Unknown)
        }

        HirExprKind::Call { .. } => Type::Unknown,

        HirExprKind::Assign { .. } => Type::Unit,

        HirExprKind::If { .. } => Type::Unknown,
    }
}

/// Infer the type of a literal
fn infer_literal_type(lit: &kivc_ast::Literal) -> Type {
    match lit {
        kivc_ast::Literal::Int(_) => Type::Int,
        kivc_ast::Literal::Float(_) => Type::Float,
        kivc_ast::Literal::Bool(_) => Type::Bool,
        kivc_ast::Literal::Text(_) => Type::Text,
        kivc_ast::Literal::Unit => Type::Unit,
    }
}

/// Convert a Type to a TypeId
pub fn type_to_type_id(ty: &Type) -> Option<TypeId> {
    match ty {
        Type::Int => Some(TypeId::new(0)),
        Type::Float => Some(TypeId::new(1)),
        Type::Bool => Some(TypeId::new(2)),
        Type::Text => Some(TypeId::new(3)),
        Type::Unit => Some(TypeId::new(4)),
        Type::Unknown => None,
    }
}
