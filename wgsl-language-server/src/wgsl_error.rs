use lsp_types::{Diagnostic, Position, Range};
use naga::{front::wgsl::ParseError, valid::ValidationError, SourceLocation, WithSpan};

#[derive(Debug)]
pub struct WgslError {
    error: String,
    location: Option<SourceLocation>,
    src: String,
}

pub fn source_location_to_range(location: Option<SourceLocation>) -> Option<Range> {
    location.map(|loc| {
        Range::new(
            Position::new(loc.line_number - 1, loc.line_position - 1),
            Position::new(loc.line_number - 1, loc.line_position + loc.length - 1),
        )
    })
}

impl WgslError {
    pub fn from_validation_err(err: &WithSpan<ValidationError>, src: &str) -> Self {
        Self {
            error: err.emit_to_string(src),
            location: err.location(src),
            src: src.to_owned(),
        }
    }

    pub fn from_parse_err(err: &ParseError, src: &str) -> Self {
        Self {
            error: err.emit_to_string(src),
            location: err.location(src),
            src: src.to_owned(),
        }
    }
}

impl From<WgslError> for Diagnostic {
    fn from(val: WgslError) -> Self {
        Diagnostic {
            message: val.error,
            range: source_location_to_range(val.location).unwrap_or_default(),
            source: Some(val.src),
            ..Default::default()
        }
    }
}
