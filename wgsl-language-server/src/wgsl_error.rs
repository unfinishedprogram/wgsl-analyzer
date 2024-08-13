use codespan_reporting::diagnostic::{Diagnostic, LabelStyle};
use lsp_types::{DiagnosticRelatedInformation, Url};
use naga::{front::wgsl::ParseError, SourceLocation};

use crate::{
    pretty_error::error_context::ErrorContext,
    range_tools::{new_location, range_to_span, source_location_to_range, span_to_lsp_range},
};

pub fn codespan_to_lsp_diagnostic(
    diagnostic: codespan_reporting::diagnostic::Diagnostic<()>,
    location: Option<SourceLocation>,
    url: &Url,
    src: &str,
) -> lsp_types::Diagnostic {
    let range = if let Some(location) = location {
        source_location_to_range(Some(location), src).unwrap_or_default()
    } else {
        span_to_lsp_range(
            diagnostic
                .labels
                .first()
                .map(|label| range_to_span(label.range.clone()))
                .unwrap_or_default(),
            src,
        )
    };

    let message = if diagnostic.message.is_empty() {
        diagnostic
            .labels
            .first()
            .map(|label| label.message.clone())
            .unwrap_or_default()
    } else {
        diagnostic.message
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
        message,
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
    let labels = err
        .labels()
        .map(|(span, msg)| {
            codespan_reporting::diagnostic::Label::new(
                LabelStyle::Primary,
                (),
                span.to_range().unwrap(),
            )
            .with_message(msg)
        })
        .collect();

    let location = err.location(src);
    let diagnostic = Diagnostic::error()
        .with_labels(labels)
        .with_message(err.message())
        .with_notes(vec!["PARSE_ERROR".to_string()]);

    codespan_to_lsp_diagnostic(diagnostic, location, url, src)
}

pub fn validation_error_to_codespan_diagnostic(
    err: &naga::WithSpan<naga::valid::ValidationError>,
    src: &str,
    module: &naga::Module,
) -> codespan_reporting::diagnostic::Diagnostic<()> {
    let ctx = ErrorContext::new(module, src);

    ctx.validation_error_diagnostic(err)
        .with_notes(vec!["VALIDATION_ERROR".to_string()])
}

pub fn validation_error_to_lsp_diagnostic(
    err: &naga::WithSpan<naga::valid::ValidationError>,
    src: &str,
    url: &lsp_types::Url,
    module: &naga::Module,
) -> lsp_types::Diagnostic {
    let location = err.location(src);
    codespan_to_lsp_diagnostic(
        validation_error_to_codespan_diagnostic(err, src, module),
        location,
        url,
        src,
    )
}
