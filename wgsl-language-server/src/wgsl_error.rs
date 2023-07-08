use codespan_reporting::diagnostic::LabelStyle;
use lsp_types::DiagnosticRelatedInformation;
use naga::{front::wgsl::ParseError, valid::ValidationError, WithSpan};

use crate::range_tools::{new_location, source_location_to_range};

#[derive(Debug)]
pub struct WgslError {
    error: String,
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
        let diagnostic = err.diagnostic();

        let mut related_information = vec![];

        let mut location = diagnostic
            .labels
            .first()
            .map(|loc| new_location(loc.range.clone(), src, path.to_owned()));

        for label in diagnostic.labels {
            if matches!(label.style, LabelStyle::Primary) {
                location = Some(new_location(label.range.to_owned(), src, path.to_owned()));
            }

            related_information.push(DiagnosticRelatedInformation {
                location: new_location(label.range, src, path.to_owned()),
                message: label.message,
            })
        }

        Self {
            error: err.emit_to_string_with_path(src, path.as_str()),
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
            error: err.emit_to_string_with_path(src, path.as_str()),
            location: source_location_to_range(err.location(src), src),
            related_information,
        }
    }
}

impl From<WgslError> for lsp_types::Diagnostic {
    fn from(val: WgslError) -> Self {
        lsp_types::Diagnostic {
            message: val.error,
            range: val.location.unwrap_or_default(),
            source: Some("wgsl-language-support".to_owned()),
            related_information: Some(val.related_information),
            ..Default::default()
        }
    }
}
