//! Program-level name resolution.

use crate::resolver::Resolver;
use kivc_hir::HirProgram;

impl Resolver {
    /// Resolve program in-place, modifying the program directly
    pub fn resolve_program_inplace(&mut self, program: &mut HirProgram) {
        // First pass: register all function names using scope manager
        for function in &program.functions {
            let symbol = crate::scope::Symbol::function(
                function.name.clone(),
                function.fun_id,
                function.span.clone(),
            );
            if let Err((_msg, original_span)) = self.scope_manager.insert(symbol) {
                // Create multi-span error showing both declarations
                let error = kivc_diagnostics::KivError::multi_span(
                    &function.span,
                    format!("function '{}' is already declared", function.name),
                    "redeclared here",
                    vec![(&original_span, "originally declared here".to_string())],
                    Some("use a different name or remove one of the declarations".to_string()),
                    kivc_diagnostics::error_code::E001_UNEXPECTED_TOKEN,
                );
                self.diagnostics.add(error);
            } else {
                // Also register in function_names for backward compatibility
                self.function_names
                    .insert(function.name.clone(), function.fun_id);
            }
        }

        // Second pass: resolve function bodies in-place
        for function in &mut program.functions {
            self.resolve_function_inplace(function);
        }
    }
}
