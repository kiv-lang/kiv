//! Block-level name resolution.

use crate::resolver::Resolver;
use kivc_hir::HirBlock;

impl Resolver {
    /// Resolve block in-place
    pub(crate) fn resolve_block_inplace(&mut self, block: &mut HirBlock) {
        // Enter block scope
        self.scope_manager.enter_scope();

        for stmt in &mut block.stmts {
            self.resolve_stmt_inplace(stmt);
        }

        // Exit block scope
        self.scope_manager.exit_scope();
    }
}
