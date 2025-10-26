//! Compilation error type for pipeline stages.

use std::fmt;

/// Error type indicating a compilation stage has failed.
///
/// This is used by compilation stages that report detailed errors through
/// `DiagnosticsCollector`. The error itself carries no additional information,
/// as all error details are collected in the diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompilationFailed;

impl fmt::Display for CompilationFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "compilation failed")
    }
}

impl std::error::Error for CompilationFailed {}
