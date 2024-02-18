pub mod severity;
use severity::Severity;

use ariadne::{Label, Report};
use chumsky::span::SimpleSpan;
use std::ops::Range;

pub struct Diagnostic {
    pub severity: Severity,
    pub span: SimpleSpan,
    pub message: String,
    pub related_info: Vec<DiagnosticRelatedInfo>,
}

pub struct DiagnosticRelatedInfo {
    pub span: SimpleSpan,
    pub message: String,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>, span: SimpleSpan) -> Self {
        Self {
            severity: Severity::Error,
            span,
            message: message.into(),
            related_info: Vec::new(),
        }
    }

    pub fn related(mut self, message: impl Into<String>, span: SimpleSpan) -> Self {
        let info = DiagnosticRelatedInfo {
            span,
            message: message.into(),
        };
        self.related_info.push(info);
        self
    }

    pub fn build_report(self, path: &str) -> Report<(&str, Range<usize>)> {
        Report::build(ariadne::ReportKind::Error, path, self.span.start)
            .with_label(
                Label::new((path, self.span.into_range()))
                    .with_message(self.message)
                    .with_color(self.severity.into()),
            )
            .with_labels(self.related_info.into_iter().map(|info| {
                Label::new((path, info.span.into_range()))
                    .with_message(info.message)
                    .with_color(self.severity.into())
            }))
            .finish()
    }
}
