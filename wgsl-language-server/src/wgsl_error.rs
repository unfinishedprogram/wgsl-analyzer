use lsp_types::DiagnosticRelatedInformation;
use naga::{front::wgsl::ParseError, valid::ValidationError, SourceLocation, WithSpan};

use crate::range_tools::{new_location, source_location_to_range};

#[derive(Debug)]
pub struct WgslError {
    error: String,
    location: Option<SourceLocation>,
    src: String,
    related_information: Vec<DiagnosticRelatedInformation>,
}

impl WgslError {
    pub fn from_validation_err(
        err: &WithSpan<ValidationError>,
        src: &str,
        path: &lsp_types::Url,
    ) -> Self {
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
            location: err.location(src),
            src: src.to_owned(),
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
            location: err.location(src),
            src: src.to_owned(),
            related_information,
        }
    }
}

impl From<WgslError> for lsp_types::Diagnostic {
    fn from(val: WgslError) -> Self {
        lsp_types::Diagnostic {
            message: val.error,
            range: source_location_to_range(val.location, &val.src).unwrap_or_default(),
            source: Some("wgsl-language-support".to_owned()),
            related_information: Some(val.related_information),
            ..Default::default()
        }
    }
}
