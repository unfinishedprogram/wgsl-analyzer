use codespan_reporting::diagnostic::Diagnostic;
use lsp_types::DiagnosticRelatedInformation;
use naga::{
    front::wgsl::ParseError,
    valid::{NestedSpan, ValidationError},
    WithSpan,
};

use crate::{
    range_tools::{new_location, source_location_to_range},
    span_tools::spans_as_labels,
};

#[derive(Debug)]
pub struct WgslError {
    location: Option<lsp_types::Range>,
    related_information: Vec<DiagnosticRelatedInformation>,
}

// Type mostly for conversions between Naga errors and LSP errors
impl WgslError {
    pub fn from_validation_err(
        err: &WithSpan<ValidationError>,
        src: &str,
        path: &lsp_types::Url,
    ) -> Self {
        let diagnostic = Diagnostic::error().with_labels(spans_as_labels(err.nested_spans()));

        let mut related_information = vec![];

        let location = diagnostic
            .labels
            .first()
            .as_ref()
            .map(|loc| new_location(loc.range.clone(), src, path.to_owned()));

        for label in diagnostic.labels.iter() {
            related_information.push(DiagnosticRelatedInformation {
                location: new_location(label.range.clone(), src, path.to_owned()),
                message: label.message.clone(),
            })
        }

        Self {
            location: location.map(|v| v.range),
            related_information,
        }
    }

    pub fn from_parse_err(err: &ParseError, src: &str, path: &lsp_types::Url) -> Self {
        let diagnostic = err.diagnostic();
        let mut related_information = vec![];

        for label in diagnostic.labels {
            related_information.push(DiagnosticRelatedInformation {
                location: new_location(label.range, src, path.to_owned()),
                message: label.message,
            })
        }

        Self {
            location: source_location_to_range(err.location(src), src),
            related_information,
        }
    }

    pub fn diagnostics_list(&self) -> Vec<lsp_types::Diagnostic> {
        let diagnostics = self
            .related_information
            .iter()
            .map(|info| lsp_types::Diagnostic {
                message: info.message.clone(),
                range: self.location.unwrap_or_default(),
                source: Some("wgsl-language-support".to_owned()),
                ..Default::default()
            })
            .collect();
        diagnostics
    }
}
