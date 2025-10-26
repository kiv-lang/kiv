//! Scope management for name resolution.

use kivc_hir::{FunId, VarId};
use kivc_span::Span;
use std::collections::HashMap;

/// Unique identifier for a scope
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(usize);

impl ScopeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

/// The kind of symbol in a scope
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Variable(VarId),
    Function(FunId),
    Constant(VarId),
}

/// A symbol in a scope
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub is_mutable: bool,
    pub span: Span,
}

impl Symbol {
    pub fn variable(name: String, var_id: VarId, is_mutable: bool, span: Span) -> Self {
        Self {
            name,
            kind: SymbolKind::Variable(var_id),
            is_mutable,
            span,
        }
    }

    pub fn function(name: String, fun_id: FunId, span: Span) -> Self {
        Self {
            name,
            kind: SymbolKind::Function(fun_id),
            is_mutable: false,
            span,
        }
    }

    pub fn constant(name: String, var_id: VarId, span: Span) -> Self {
        Self {
            name,
            kind: SymbolKind::Constant(var_id),
            is_mutable: false,
            span,
        }
    }
}

/// A lexical scope for name resolution
#[derive(Debug, Clone)]
pub struct Scope {
    pub id: ScopeId,
    pub parent: Option<ScopeId>,
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    pub fn new(id: ScopeId, parent: Option<ScopeId>) -> Self {
        Self {
            id,
            parent,
            symbols: HashMap::new(),
        }
    }

    /// Insert a symbol into this scope
    /// Returns the existing symbol if the name is already taken
    pub fn insert(&mut self, symbol: Symbol) -> Result<(), (String, Span)> {
        if let Some(existing) = self.symbols.get(&symbol.name) {
            Err((
                format!("Symbol '{}' already defined in this scope", symbol.name),
                existing.span.clone(),
            ))
        } else {
            self.symbols.insert(symbol.name.clone(), symbol);
            Ok(())
        }
    }

    /// Lookup a symbol in this scope only
    pub fn lookup_local(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Get all symbols in this scope
    pub fn symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.values()
    }
}

/// Manages multiple scopes and provides scope chain lookups
#[derive(Debug, Clone)]
pub struct ScopeManager {
    scopes: Vec<Scope>,
    current_scope: ScopeId,
    next_scope_id: usize,
}

impl ScopeManager {
    pub fn new() -> Self {
        let root_scope = Scope::new(ScopeId::new(0), None);
        Self {
            scopes: vec![root_scope],
            current_scope: ScopeId::new(0),
            next_scope_id: 1,
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self) -> ScopeId {
        let new_scope_id = ScopeId::new(self.next_scope_id);
        self.next_scope_id += 1;

        let new_scope = Scope::new(new_scope_id, Some(self.current_scope));
        self.scopes.push(new_scope);
        self.current_scope = new_scope_id;

        new_scope_id
    }

    /// Exit the current scope and return to the parent
    pub fn exit_scope(&mut self) {
        if let Some(scope) = self.scopes.iter().find(|s| s.id == self.current_scope)
            && let Some(parent_id) = scope.parent
        {
            self.current_scope = parent_id;
        }
    }

    /// Insert a symbol into the current scope
    pub fn insert(&mut self, symbol: Symbol) -> Result<(), (String, Span)> {
        if let Some(scope) = self.scopes.iter_mut().find(|s| s.id == self.current_scope) {
            scope.insert(symbol)
        } else {
            Err(("Current scope not found".to_string(), symbol.span.clone()))
        }
    }

    /// Lookup a symbol in the current scope chain
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut current = Some(self.current_scope);

        while let Some(scope_id) = current {
            if let Some(scope) = self.scopes.iter().find(|s| s.id == scope_id) {
                if let Some(symbol) = scope.lookup_local(name) {
                    return Some(symbol);
                }
                current = scope.parent;
            } else {
                break;
            }
        }

        None
    }

    /// Get the current scope
    #[allow(dead_code)]
    pub fn current_scope(&self) -> &Scope {
        self.scopes
            .iter()
            .find(|s| s.id == self.current_scope)
            .unwrap()
    }
}

impl Default for ScopeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kivc_span::SourceFile;
    use std::sync::Arc;

    fn dummy_span() -> Span {
        let file = SourceFile::new("test.kiv".to_string(), "".to_string());
        Span::new(Arc::new(file), 0.into(), 0.into())
    }

    #[test]
    fn test_scope_insert_and_lookup() {
        let mut scope = Scope::new(ScopeId::new(0), None);
        let symbol = Symbol::variable("x".to_string(), VarId::new(0), false, dummy_span());

        assert!(scope.insert(symbol).is_ok());
        assert!(scope.lookup_local("x").is_some());
        assert!(scope.lookup_local("y").is_none());
    }

    #[test]
    fn test_scope_duplicate_insert() {
        let mut scope = Scope::new(ScopeId::new(0), None);
        let symbol1 = Symbol::variable("x".to_string(), VarId::new(0), false, dummy_span());
        let symbol2 = Symbol::variable("x".to_string(), VarId::new(1), false, dummy_span());

        assert!(scope.insert(symbol1).is_ok());
        assert!(scope.insert(symbol2).is_err());
    }

    #[test]
    fn test_scope_manager() {
        let mut manager = ScopeManager::new();

        // Insert in root scope
        manager
            .insert(Symbol::variable("x".to_string(), VarId::new(0), false, dummy_span()))
            .unwrap();
        assert!(manager.lookup("x").is_some());

        // Enter new scope
        manager.enter_scope();
        manager
            .insert(Symbol::variable("y".to_string(), VarId::new(1), false, dummy_span()))
            .unwrap();

        // Can lookup both x (from parent) and y (from current)
        assert!(manager.lookup("x").is_some());
        assert!(manager.lookup("y").is_some());

        // Exit scope
        manager.exit_scope();

        // Can still lookup x but not y
        assert!(manager.lookup("x").is_some());
        assert!(manager.lookup("y").is_none());
    }
}
