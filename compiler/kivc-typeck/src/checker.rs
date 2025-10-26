//! Type checker core implementation.

use crate::types::{Type, TypeContext};
use kivc_diagnostics::DiagnosticsCollector;
use kivc_hir::{HirBlock, HirExpr, HirFunDef, HirProgram, HirStmt, TypeId, VarId};
use std::collections::HashMap;

/// The type checking context
pub struct TypeChecker {
    pub(crate) ctx: TypeContext,
    pub(crate) var_types: HashMap<VarId, Type>,
    pub(crate) var_mutability: HashMap<VarId, bool>,
    pub(crate) current_return_type: Option<TypeId>,
    pub(crate) diagnostics: DiagnosticsCollector,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            ctx: TypeContext::new(),
            var_types: HashMap::new(),
            var_mutability: HashMap::new(),
            current_return_type: None,
            diagnostics: DiagnosticsCollector::new(),
        }
    }

    /// Check program in-place
    pub fn check_program_inplace(&mut self, program: &mut HirProgram) {
        for function in &mut program.functions {
            self.check_function_inplace(function);
        }
    }

    /// Check function in-place
    pub fn check_function_inplace(&mut self, fun: &mut HirFunDef) {
        // Store parameter types
        for param in &fun.params {
            if let Some(ty) = self.ctx.get(param.ty) {
                self.var_types.insert(param.var_id, ty.clone());
                self.var_mutability.insert(param.var_id, false);
            }
        }

        // Store return type for validation
        self.current_return_type = fun.return_type;

        // Check function body in-place
        self.check_block_inplace(&mut fun.body);

        // Clear function-specific state
        self.var_types.clear();
        self.var_mutability.clear();
        self.current_return_type = None;
    }

    /// Check block in-place
    pub fn check_block_inplace(&mut self, block: &mut HirBlock) {
        for stmt in &mut block.stmts {
            self.check_stmt_inplace(stmt);
        }
    }

    /// Check statement in-place
    pub fn check_stmt_inplace(&mut self, stmt: &mut HirStmt) {
        crate::stmt::check_stmt_inplace(self, stmt)
    }

    /// Check expression in-place
    pub fn check_expr_inplace(&mut self, expr: &mut HirExpr) {
        crate::expr::check_expr_inplace(self, expr)
    }

    pub fn infer_expr_type(&self, expr: &HirExpr) -> Type {
        crate::inference::infer_expr_type(self, expr)
    }

    pub fn type_to_type_id(&self, ty: &Type) -> Option<TypeId> {
        crate::inference::type_to_type_id(ty)
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
