//! Diagnostic and error reporting system for the Kiv compiler.
//!
//! This crate provides structured error types with rich formatting capabilities
//! powered by miette, including source code snippets, labels, and helpful suggestions.

mod collector;
mod compile_error;
mod error;

pub mod error_code;

pub use collector::{Diagnostic, DiagnosticLevel, DiagnosticsCollector};
pub use compile_error::CompilationFailed;
pub use error::KivError;
pub use error_code::ErrorCode;

// Re-export commonly used types from miette
pub use miette::{Diagnostic as MietteDiagnostic, Report, Result};

/// Convenience methods for common error patterns
impl DiagnosticsCollector {
    /// Report a syntax error
    pub fn syntax_error(&mut self, span: &kivc_span::Span, msg: impl Into<String>) {
        self.add(KivError::syntax(
            span,
            msg,
            "syntax error",
            None,
            error_code::E001_UNEXPECTED_TOKEN,
        ));
    }

    /// Report a type error
    pub fn type_error(&mut self, span: &kivc_span::Span, msg: impl Into<String>) {
        self.add(KivError::type_error(
            span,
            msg,
            "type error",
            None,
            error_code::E100_TYPE_MISMATCH,
        ));
    }

    /// Report a warning
    pub fn warning_msg(&mut self, span: &kivc_span::Span, msg: impl Into<String>) {
        self.add(KivError::warning(span.clone(), msg, "warning"));
    }
}
