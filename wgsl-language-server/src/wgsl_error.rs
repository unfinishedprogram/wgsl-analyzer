use lsp_types::{DiagnosticRelatedInformation, Url};
use naga::{front::wgsl::ParseError, SourceLocation};

use crate::range_tools::{new_location, range_to_span, source_location_to_range, span_to_range};

pub fn codespan_to_lsp_diagnostic(
    diagnostic: codespan_reporting::diagnostic::Diagnostic<()>,
    location: Option<SourceLocation>,
    url: &Url,
    src: &str,
) -> lsp_types::Diagnostic {
    let range = if let Some(location) = location {
        source_location_to_range(Some(location), src).unwrap_or_default()
    } else {
        span_to_range(
            diagnostic
                .labels
                .first()
                .map(|label| range_to_span(label.range.clone()))
                .unwrap(),
            src,
        )
    };

    let lsp_location = lsp_types::Location::new(url.clone(), range);

    let mut related_information = vec![];

    for label in diagnostic.labels {
        related_information.push(DiagnosticRelatedInformation {
            location: new_location(label.range, src, url.to_owned()),
            message: label.message,
        })
    }

    for note in diagnostic.notes {
        related_information.push(DiagnosticRelatedInformation {
            location: lsp_location.clone(),
            message: note,
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

pub fn parse_error_to_lsp_diagnostic(
    err: &ParseError,
    src: &str,
    url: &lsp_types::Url,
) -> lsp_types::Diagnostic {
    let diagnostic = err.diagnostic();
    let location = err.location(src);
    codespan_to_lsp_diagnostic(diagnostic, location, url, src)
}
