//! Core resolver state and initialization.

use crate::scope::ScopeManager;
use kivc_diagnostics::DiagnosticsCollector;
use kivc_hir::FunId;
use std::collections::HashMap;

/// The name resolver
pub struct Resolver {
    pub(crate) scope_manager: ScopeManager,
    pub diagnostics: DiagnosticsCollector,
    pub(crate) function_names: HashMap<String, FunId>,
}

impl Resolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            scope_manager: ScopeManager::new(),
            diagnostics: DiagnosticsCollector::new(),
            function_names: HashMap::new(),
        };
        
        // Register built-in functions
        resolver.register_builtins();
        
        resolver
    }
    
    /// Register built-in functions that are available to all programs
    fn register_builtins(&mut self) {
        use kivc_span::{SourceFile, Span};
        
        // Use a special sentinel FunId for built-in functions
        // Built-ins don't have actual HIR function definitions
        let builtin_id = FunId::new(usize::MAX);
        
        // Create a dummy span for built-in functions
        let builtin_file = SourceFile::new("<builtin>".to_string(), "".to_string());
        let builtin_span = Span::new(builtin_file, 0.into(), 0.into());
        
        let builtins = vec!["print", "panic"];
        
        for name in builtins {
            self.function_names.insert(name.to_string(), builtin_id);
            let symbol = crate::scope::Symbol::function(name.to_string(), builtin_id, builtin_span.clone());
            // Ignore errors since we control these names
            let _ = self.scope_manager.insert(symbol);
        }
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}
