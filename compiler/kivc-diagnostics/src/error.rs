use crate::error_code::ErrorCode;
use kivc_span::Span;
use miette::{Diagnostic, SourceSpan};
use std::fmt;
use std::sync::Arc;
use thiserror::Error;

/// Main error type for the Kiv compiler.
///
/// Designed to work seamlessly with miette for beautiful error reporting.
#[derive(Clone, Error)]
pub enum KivError {
    /// Syntax error with source location
    #[error("{message}")]
    Syntax {
        src: Arc<dyn miette::SourceCode + Send + Sync>,
        span: SourceSpan,
        message: String,
        label: String,
        help: Option<String>,
        code: ErrorCode,
    },

    /// Type error with source location
    #[error("{message}")]
    Type {
        src: Arc<dyn miette::SourceCode + Send + Sync>,
        span: SourceSpan,
        message: String,
        label: String,
        help: Option<String>,
        code: ErrorCode,
    },

    /// IO error without source location
    #[error("{message}")]
    Io {
        message: String,
        help: Option<String>,
        code: ErrorCode,
    },

    /// Generic error without source location
    #[error("{message}")]
    Generic {
        message: String,
        help: Option<String>,
    },

    /// Multi-span error with multiple labeled locations
    #[error("{message}")]
    MultiSpan {
        src: Arc<dyn miette::SourceCode + Send + Sync>,
        spans: Vec<(SourceSpan, String)>,
        message: String,
        help: Option<String>,
        code: ErrorCode,
    },
}

impl KivError {
    /// Creates a syntax error
    pub fn syntax(
        span: &Span,
        message: impl Into<String>,
        label: impl Into<String>,
        help: Option<String>,
        code: ErrorCode,
    ) -> Self {
        let file = span.file();
        let source = miette::NamedSource::new(file.name(), file.source().to_string());

        Self::Syntax {
            src: Arc::new(source),
            span: SourceSpan::new(
                (u32::from(span.offset()) as usize).into(),
                u32::from(span.len()) as usize,
            ),
            message: message.into(),
            label: label.into(),
            help,
            code,
        }
    }

    /// Creates a type error
    pub fn type_error(
        span: &Span,
        message: impl Into<String>,
        label: impl Into<String>,
        help: Option<String>,
        code: ErrorCode,
    ) -> Self {
        let file = span.file();
        let source = miette::NamedSource::new(file.name(), file.source().to_string());

        Self::Type {
            src: Arc::new(source),
            span: SourceSpan::new(
                (u32::from(span.offset()) as usize).into(),
                u32::from(span.len()) as usize,
            ),
            message: message.into(),
            label: label.into(),
            help,
            code,
        }
    }

    /// Creates an IO error
    pub fn io(message: impl Into<String>, help: Option<String>, code: ErrorCode) -> Self {
        Self::Io {
            message: message.into(),
            help,
            code,
        }
    }

    /// Creates a generic error
    pub fn generic(message: impl Into<String>, help: Option<String>) -> Self {
        Self::Generic {
            message: message.into(),
            help,
        }
    }

    /// Creates a warning (represented as a syntax error with a special code)
    pub fn warning(span: Span, message: impl Into<String>, help: impl Into<String>) -> Self {
        Self::syntax(
            &span,
            message,
            "warning",
            Some(help.into()),
            ErrorCode::new("lint", 0),
        )
    }

    /// Creates a multi-span error showing multiple related locations
    pub fn multi_span(
        primary_span: &Span,
        message: impl Into<String>,
        primary_label: impl Into<String>,
        related_spans: Vec<(&Span, String)>,
        help: Option<String>,
        code: ErrorCode,
    ) -> Self {
        let file = primary_span.file();
        let source = miette::NamedSource::new(file.name(), file.source().to_string());

        // Create the primary span
        let primary_source_span = SourceSpan::new(
            (u32::from(primary_span.offset()) as usize).into(),
            u32::from(primary_span.len()) as usize,
        );

        // Create related spans
        let mut all_spans = vec![(primary_source_span, primary_label.into())];
        all_spans.extend(related_spans.into_iter().map(|(span, label)| {
            (
                SourceSpan::new(
                    (u32::from(span.offset()) as usize).into(),
                    u32::from(span.len()) as usize,
                ),
                label,
            )
        }));

        Self::MultiSpan {
            src: Arc::new(source),
            spans: all_spans,
            message: message.into(),
            help,
            code,
        }
    }
}

// Manual implementation of Diagnostic trait for KivError
impl Diagnostic for KivError {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            Self::Syntax { code, .. }
            | Self::Type { code, .. }
            | Self::Io { code, .. }
            | Self::MultiSpan { code, .. } => Some(Box::new(code.clone())),
            Self::Generic { .. } => None,
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        match self {
            Self::Syntax { help, .. }
            | Self::Type { help, .. }
            | Self::Io { help, .. }
            | Self::Generic { help, .. }
            | Self::MultiSpan { help, .. } => help.as_ref().map(|h| Box::new(h.as_str()) as Box<dyn fmt::Display>),
        }
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        match self {
            Self::Syntax { src, .. } | Self::Type { src, .. } | Self::MultiSpan { src, .. } => {
                Some(&**src as &dyn miette::SourceCode)
            }
            Self::Io { .. } | Self::Generic { .. } => None,
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        match self {
            Self::Syntax { span, label, .. } => Some(Box::new(std::iter::once(
                miette::LabeledSpan::new_with_span(Some(label.clone()), *span),
            ))),
            Self::Type { span, label, .. } => Some(Box::new(std::iter::once(
                miette::LabeledSpan::new_with_span(Some(label.clone()), *span),
            ))),
            Self::MultiSpan { spans, .. } => Some(Box::new(
                spans
                    .iter()
                    .map(|(span, label)| miette::LabeledSpan::new_with_span(Some(label.clone()), *span)),
            )),
            Self::Io { .. } | Self::Generic { .. } => None,
        }
    }
}

// Manual Debug implementation to handle the non-Debug SourceCode trait object
impl fmt::Debug for KivError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax {
                message,
                label,
                help,
                code,
                ..
            } => f
                .debug_struct("Syntax")
                .field("message", message)
                .field("label", label)
                .field("help", help)
                .field("code", code)
                .finish_non_exhaustive(),
            Self::Type {
                message,
                label,
                help,
                code,
                ..
            } => f
                .debug_struct("Type")
                .field("message", message)
                .field("label", label)
                .field("help", help)
                .field("code", code)
                .finish_non_exhaustive(),
            Self::Io {
                message,
                help,
                code,
            } => f
                .debug_struct("Io")
                .field("message", message)
                .field("help", help)
                .field("code", code)
                .finish(),
            Self::Generic { message, help } => f
                .debug_struct("Generic")
                .field("message", message)
                .field("help", help)
                .finish(),
            Self::MultiSpan {
                message,
                spans,
                help,
                code,
                ..
            } => f
                .debug_struct("MultiSpan")
                .field("message", message)
                .field("spans_count", &spans.len())
                .field("help", help)
                .field("code", code)
                .finish_non_exhaustive(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_code::*;
    use kivc_span::SourceFile;
    use miette::Report;

    #[test]
    fn test_syntax_error() {
        let file = SourceFile::new("test.kiv".to_string(), "let x = 42;".to_string());
        let span = Span::new(Arc::clone(&file), 4.into(), 1.into());

        let error = KivError::syntax(
            &span,
            "unexpected token",
            "expected identifier",
            Some("try using a valid identifier name".to_string()),
            E001_UNEXPECTED_TOKEN,
        );

        assert!(matches!(error, KivError::Syntax { .. }));
    }

    #[test]
    fn test_type_error() {
        let file = SourceFile::new("test.kiv".to_string(), "let x: Int = true;".to_string());
        let span = Span::new(Arc::clone(&file), 13.into(), 4.into());

        let error = KivError::type_error(
            &span,
            "type mismatch",
            "expected Int, found Bool",
            Some("consider changing the type annotation".to_string()),
            E100_TYPE_MISMATCH,
        );

        assert!(matches!(error, KivError::Type { .. }));
    }

    #[test]
    fn test_io_error() {
        let error = KivError::io(
            "file not found",
            Some("check that the file exists".to_string()),
            E200_FILE_NOT_FOUND,
        );

        assert!(matches!(error, KivError::Io { .. }));
    }

    #[test]
    fn test_generic_error() {
        let error = KivError::generic("something went wrong", None);
        assert!(matches!(error, KivError::Generic { .. }));
    }

    #[test]
    fn test_error_can_be_converted_to_report() {
        let file = SourceFile::new("test.kiv".to_string(), "let x = 42;".to_string());
        let span = Span::new(Arc::clone(&file), 4.into(), 1.into());

        let error = KivError::syntax(
            &span,
            "unexpected token",
            "here",
            None,
            E001_UNEXPECTED_TOKEN,
        );

        let _report: Report = error.into();
    }
}
