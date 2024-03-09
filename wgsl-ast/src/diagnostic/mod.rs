pub mod severity;
use severity::Severity;

use chumsky::span::SimpleSpan;

use crate::front::ast::ModuleError;

#[derive(Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub span: Option<SimpleSpan>,
    pub message: String,
    pub related_info: Vec<DiagnosticRelatedInfo>,
}

#[derive(Clone)]
pub struct DiagnosticRelatedInfo {
    pub span: SimpleSpan,
    pub message: String,
}

impl From<&ModuleError<'_>> for Diagnostic {
    fn from(err: &ModuleError) -> Self {
        Self {
            severity: Severity::Error,
            span: Some(*err.span()),
            message: err.message(),
            related_info: Vec::new(),
        }
    }
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            span: None,
            message: message.into(),
            related_info: Vec::new(),
        }
    }

    pub fn span_if_none(mut self, span: SimpleSpan) -> Self {
        if self.span.is_none() {
            self.span = Some(span);
        }
        self
    }

    pub fn span(mut self, span: SimpleSpan) -> Self {
        self.span = Some(span);
        self
    }

    pub fn related(mut self, message: impl Into<String>, span: SimpleSpan) -> Self {
        let info = DiagnosticRelatedInfo {
            span,
            message: message.into(),
        };
        self.related_info.push(info);
        self
    }
}

// This lets us use the `?` syntax, with functions that only have a single point of failure more conveniently
impl From<Diagnostic> for Vec<Diagnostic> {
    fn from(diag: Diagnostic) -> Self {
        vec![diag]
    }
}
