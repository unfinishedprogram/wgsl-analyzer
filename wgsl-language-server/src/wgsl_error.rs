use codespan_reporting::diagnostic::LabelStyle;
use lsp_types::DiagnosticRelatedInformation;
use naga::{front::wgsl::ParseError, valid::ValidationError, WithSpan};

use crate::{
    pretty_error::error_context::ErrorContext,
    range_tools::{new_location, source_location_to_range},
};

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
        context: Option<ErrorContext>,
    ) -> Self {
        let diagnostic = if let Some(context) = context {
            context.get_diagnostic(err)
        } else {
            err.diagnostic()
        };

        let mut related_information = vec![];

        let mut location = diagnostic
            .labels
            .first()
            .as_ref()
            .map(|loc| new_location(loc.range.clone(), src, path.to_owned()));

        for label in diagnostic.labels.iter() {
            if matches!(label.style, LabelStyle::Primary) {
                location = Some(new_location(label.range.to_owned(), src, path.to_owned()));
            }

            related_information.push(DiagnosticRelatedInformation {
                location: new_location(label.range.clone(), src, path.to_owned()),
                message: label.message.clone(),
            })
        }

        Self {
            error: emit_diagnostic_to_string(&diagnostic, path.as_str(), src),
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

pub fn emit_diagnostic_to_string(
    diagnostic: &codespan_reporting::diagnostic::Diagnostic<()>,
    path: &str,
    source: &str,
) -> String {
    use codespan_reporting::{files, term};
    use term::termcolor::NoColor;

    let files = files::SimpleFile::new(path, source);
    let config = codespan_reporting::term::Config::default();
    let mut writer = NoColor::new(Vec::new());
    term::emit(&mut writer, &config, &files, diagnostic).expect("cannot write error");
    String::from_utf8(writer.into_inner()).unwrap()
}
