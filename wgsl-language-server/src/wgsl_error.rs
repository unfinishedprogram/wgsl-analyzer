use lsp_types::Diagnostic;
use naga::{front::wgsl::ParseError, valid::ValidationError, SourceLocation, WithSpan};

use crate::range_tools::source_location_to_range;

#[derive(Debug)]
pub struct WgslError {
    error: String,
    location: Option<SourceLocation>,
    src: String,
}

impl WgslError {
    pub fn from_validation_err(err: &WithSpan<ValidationError>, src: &str, path: &str) -> Self {
        Self {
            error: err.emit_to_string_with_path(src, path),
            location: err.location(src),
            src: src.to_owned(),
        }
    }

    pub fn from_parse_err(err: &ParseError, src: &str, path: &str) -> Self {
        Self {
            error: err.emit_to_string_with_path(src, path),
            location: err.location(src),
            src: src.to_owned(),
        }
    }
}

impl From<WgslError> for Diagnostic {
    fn from(val: WgslError) -> Self {
        Diagnostic {
            message: val.error,
            range: source_location_to_range(val.location, &val.src).unwrap_or_default(),
            source: Some(val.src),
            ..Default::default()
        }
    }
}
