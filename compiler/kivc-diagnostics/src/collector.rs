use crate::KivError;
use std::fmt;

/// Diagnostic level (error, warning, info, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

/// A diagnostic entry with level and error
#[derive(Clone)]
pub struct Diagnostic {
    level: DiagnosticLevel,
    error: KivError,
}

impl Diagnostic {
    pub fn error(error: KivError) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            error,
        }
    }

    pub fn warning(error: KivError) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            error,
        }
    }

    pub fn info(error: KivError) -> Self {
        Self {
            level: DiagnosticLevel::Info,
            error,
        }
    }

    pub fn level(&self) -> DiagnosticLevel {
        self.level
    }

    pub fn as_error(&self) -> &KivError {
        &self.error
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.level {
            DiagnosticLevel::Error => write!(f, "error: {}", self.error),
            DiagnosticLevel::Warning => write!(f, "warning: {}", self.error),
            DiagnosticLevel::Info => write!(f, "info: {}", self.error),
        }
    }
}

impl fmt::Debug for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Diagnostic")
            .field("level", &self.level)
            .field("error", &self.error)
            .finish()
    }
}

/// A collector for multiple diagnostics.
///
/// Useful for collecting errors during compilation phases without immediately
/// aborting, allowing reporting of multiple errors at once.
#[derive(Debug, Default, Clone)]
pub struct DiagnosticsCollector {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticsCollector {
    /// Creates a new empty collector
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an error diagnostic
    pub fn error(&mut self, error: impl Into<String>) {
        self.diagnostics
            .push(Diagnostic::error(KivError::generic(error.into(), None)));
    }

    /// Adds a warning diagnostic
    pub fn warning(&mut self, warning: impl Into<String>) {
        self.diagnostics
            .push(Diagnostic::warning(KivError::generic(warning.into(), None)));
    }

    /// Adds an info diagnostic
    pub fn info(&mut self, info: impl Into<String>) {
        self.diagnostics
            .push(Diagnostic::info(KivError::generic(info.into(), None)));
    }

    /// Adds a diagnostic to the collection
    pub fn add_diagnostic(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    /// Adds an error to the collection (backwards compatibility)
    pub fn add(&mut self, error: KivError) {
        self.diagnostics.push(Diagnostic::error(error));
    }

    /// Adds an error by reference (cloning it)
    pub fn add_ref(&mut self, error: &KivError) {
        self.diagnostics.push(Diagnostic::error(error.clone()));
    }

    /// Returns true if any errors have been collected
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Error)
    }

    /// Returns true if any warnings have been collected
    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Warning)
    }

    /// Returns the number of errors collected
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count()
    }

    /// Returns the number of warnings collected
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Warning)
            .count()
    }

    /// Returns the total number of diagnostics
    pub fn count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns a reference to all diagnostics
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns a reference to the collected errors (for backwards compatibility)
    pub fn errors(&self) -> Vec<&KivError> {
        self.diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .map(|d| &d.error)
            .collect()
    }

    /// Consumes the collector and returns the collected diagnostics
    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    /// Consumes the collector and returns only errors (for backwards compatibility)
    pub fn into_errors(self) -> Vec<KivError> {
        self.diagnostics
            .into_iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .map(|d| d.error)
            .collect()
    }

    /// Clears all collected diagnostics
    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    /// Merges another collector into this one
    pub fn merge(&mut self, other: DiagnosticsCollector) {
        self.diagnostics.extend(other.diagnostics);
    }

    /// Creates a Result based on whether errors were collected
    ///
    /// Returns Ok(value) if no errors, Err(self) otherwise
    pub fn finish<T>(self, value: T) -> Result<T, Self> {
        if self.has_errors() {
            Err(self)
        } else {
            Ok(value)
        }
    }

    /// Print all diagnostics
    pub fn print_all(&self) {
        for diag in &self.diagnostics {
            eprintln!("{}", diag);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_code::*;
    use kivc_span::{SourceFile, Span};
    use std::sync::Arc;

    #[test]
    fn test_empty_collector() {
        let collector = DiagnosticsCollector::new();
        assert!(!collector.has_errors());
        assert_eq!(collector.error_count(), 0);
        assert_eq!(collector.count(), 0);
    }

    #[test]
    fn test_add_error() {
        let mut collector = DiagnosticsCollector::new();

        let file = SourceFile::new("test.kiv".to_string(), "let x = 42;".to_string());
        let span = Span::new(file.clone(), 4.into(), 1.into());

        collector.add(KivError::syntax(
            &span,
            "test error",
            "here",
            None,
            E001_UNEXPECTED_TOKEN,
        ));

        assert!(collector.has_errors());
        assert_eq!(collector.error_count(), 1);
        assert_eq!(collector.count(), 1);
    }

    #[test]
    fn test_warnings() {
        let mut collector = DiagnosticsCollector::new();

        collector.warning("unused variable");

        assert!(!collector.has_errors());
        assert!(collector.has_warnings());
        assert_eq!(collector.warning_count(), 1);
        assert_eq!(collector.count(), 1);
    }

    #[test]
    fn test_multiple_diagnostics() {
        let mut collector = DiagnosticsCollector::new();

        let file = Arc::new(SourceFile::new(
            "test.kiv".to_string(),
            "let x = 42;".to_string(),
        ));
        let span1 = Span::new(Arc::clone(&file), 0.into(), 3.into());
        let span2 = Span::new(Arc::clone(&file), 4.into(), 1.into());

        collector.add(KivError::syntax(
            &span1,
            "error 1",
            "here",
            None,
            E001_UNEXPECTED_TOKEN,
        ));
        collector.warning("warning 1");
        collector.add(KivError::syntax(
            &span2,
            "error 2",
            "here",
            None,
            E002_EXPECTED_TOKEN,
        ));

        assert_eq!(collector.count(), 3);
        assert_eq!(collector.error_count(), 2);
        assert_eq!(collector.warning_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut collector = DiagnosticsCollector::new();

        let file = SourceFile::new("test.kiv".to_string(), "let x = 42;".to_string());
        let span = Span::new(file.clone(), 4.into(), 1.into());

        collector.add(KivError::syntax(
            &span,
            "test error",
            "here",
            None,
            E001_UNEXPECTED_TOKEN,
        ));

        collector.clear();
        assert!(!collector.has_errors());
        assert_eq!(collector.count(), 0);
    }

    #[test]
    fn test_finish_success() {
        let collector = DiagnosticsCollector::new();
        let result = collector.finish(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_finish_failure() {
        let mut collector = DiagnosticsCollector::new();

        let file = SourceFile::new("test.kiv".to_string(), "let x = 42;".to_string());
        let span = Span::new(file.clone(), 4.into(), 1.into());

        collector.add(KivError::syntax(
            &span,
            "test error",
            "here",
            None,
            E001_UNEXPECTED_TOKEN,
        ));

        let result = collector.finish(42);
        assert!(result.is_err());
    }

    #[test]
    fn test_merge() {
        let mut collector1 = DiagnosticsCollector::new();
        let mut collector2 = DiagnosticsCollector::new();

        collector1.error("error 1");
        collector2.error("error 2");
        collector2.warning("warning 1");

        collector1.merge(collector2);

        assert_eq!(collector1.error_count(), 2);
        assert_eq!(collector1.warning_count(), 1);
        assert_eq!(collector1.count(), 3);
    }
}
