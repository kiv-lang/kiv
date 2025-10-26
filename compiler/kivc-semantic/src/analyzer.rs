//! Semantic analyzer implementation.

use crate::control_flow::ControlFlowAnalyzer;
use crate::liveness::LivenessAnalyzer;
use kivc_diagnostics::DiagnosticsCollector;
use kivc_hir::{HirFunDef, HirProgram};

/// The semantic analyzer
pub struct SemanticAnalyzer {
    pub diagnostics: DiagnosticsCollector,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            diagnostics: DiagnosticsCollector::new(),
        }
    }

    pub fn analyze_program(&mut self, program: &HirProgram) {
        for function in &program.functions {
            self.analyze_function(function);
        }
    }

    fn analyze_function(&mut self, function: &HirFunDef) {
        // Control flow analysis
        let mut cf_analyzer = ControlFlowAnalyzer::new();
        cf_analyzer.analyze_function(function);
        self.diagnostics.merge(cf_analyzer.diagnostics);

        // Liveness analysis
        let mut liveness_analyzer = LivenessAnalyzer::new();
        liveness_analyzer.analyze_function(function);
        self.diagnostics.merge(liveness_analyzer.diagnostics);
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
