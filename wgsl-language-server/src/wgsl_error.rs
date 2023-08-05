use lsp_types::{DiagnosticRelatedInformation, Url};
use naga::{front::wgsl::ParseError, valid::ValidationError, SourceLocation, WithSpan};

use crate::range_tools::{new_location, source_location_to_range};

pub fn codespan_to_lsp_diagnostic(
    diagnostic: codespan_reporting::diagnostic::Diagnostic<()>,
    location: Option<SourceLocation>,
    url: &Url,
    src: &str,
) -> lsp_types::Diagnostic {
    let range = source_location_to_range(location, src).unwrap_or_default();

    let lsp_location = lsp_types::Location::new(url.clone(), range);

    let mut related_information = vec![];

    for note in diagnostic.notes {
        related_information.push(DiagnosticRelatedInformation {
            location: lsp_location.clone(),
            message: note,
        })
    }

    for label in diagnostic.labels {
        related_information.push(DiagnosticRelatedInformation {
            location: new_location(label.range, src, url.to_owned()),
            message: label.message,
        })
    }

    lsp_types::Diagnostic {
        message: diagnostic.message,
        range,
        related_information: Some(related_information),
        source: Some("wgsl-language-support".to_owned()),
        ..Default::default()
    }
}

pub fn validation_error_to_lsp_diagnostic(
    err: WithSpan<ValidationError>,
    src: &str,
    url: &lsp_types::Url,
) -> lsp_types::Diagnostic {
    let diagnostic = err.diagnostic();
    let location = err.location(src);
    codespan_to_lsp_diagnostic(diagnostic, location, url, src)
}

pub fn parse_error_to_lsp_diagnostic(
    err: &ParseError,
    src: &str,
    url: &lsp_types::Url,
) -> lsp_types::Diagnostic {
    let diagnostic = err.diagnostic();
    let location = err.location(src);
    codespan_to_lsp_diagnostic(diagnostic, location, url, src)
}
