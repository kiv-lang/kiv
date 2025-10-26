//! Function-level name resolution.

use crate::resolver::Resolver;
use kivc_hir::HirFunDef;

impl Resolver {
    /// Resolve function in-place
    pub(crate) fn resolve_function_inplace(&mut self, fun: &mut HirFunDef) {
        // Enter function scope
        self.scope_manager.enter_scope();

        // Register parameters
        for param in &fun.params {
            let symbol = crate::scope::Symbol::variable(
                param.name.clone(),
                param.var_id,
                false,
                param.span.clone(),
            );
            if let Err((_msg, original_span)) = self.scope_manager.insert(symbol) {
                // Create multi-span error showing both declarations
                let error = kivc_diagnostics::KivError::multi_span(
                    &param.span,
                    format!("parameter '{}' is already declared in this scope", param.name),
                    "redeclared here",
                    vec![(&original_span, "originally declared here".to_string())],
                    Some("use a different name for this parameter".to_string()),
                    kivc_diagnostics::error_code::E001_UNEXPECTED_TOKEN,
                );
                self.diagnostics.add(error);
            }
        }

        // Resolve function body in-place
        self.resolve_block_inplace(&mut fun.body);

        // Exit function scope
        self.scope_manager.exit_scope();
    }
}
