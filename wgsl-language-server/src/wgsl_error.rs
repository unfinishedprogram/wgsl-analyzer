use codespan_reporting::diagnostic::{Diagnostic, LabelStyle};
use lsp_types::{DiagnosticRelatedInformation, Uri};
use naga::{front::wgsl::ParseError, SourceLocation};

use crate::{
    pretty_error::error_context::ErrorContext,
    range_tools::{new_location, range_to_span, source_location_to_range, span_to_lsp_range},
};

pub fn codespan_to_lsp_diagnostics(
    diagnostic: codespan_reporting::diagnostic::Diagnostic<()>,
    location: Option<SourceLocation>,
    url: &Uri,
    src: &str,
) -> Vec<lsp_types::Diagnostic> {
    let primary_label = diagnostic
        .labels
        .iter()
        .find(|it| it.style == LabelStyle::Primary)
        .or_else(|| diagnostic.labels.first());

    let range = if let Some(location) = location {
        source_location_to_range(Some(location), src).unwrap_or_default()
    } else {
        span_to_lsp_range(
            range_to_span(primary_label.map(|it| it.range.clone()).unwrap_or_default()),
            src,
        )
    };

    let message = if diagnostic.message.is_empty() {
        if let Some(label) = primary_label {
            label.message.clone()
        } else {
            "".to_string()
        }
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

    vec![lsp_types::Diagnostic {
        message,
        range,
        related_information: Some(related_information),
        source: Some("wgsl-language-support".to_owned()),
        ..Default::default()
    }]
}

pub fn parse_error_to_lsp_diagnostic(
    err: &ParseError,
    src: &str,
    url: &lsp_types::Uri,
) -> Vec<lsp_types::Diagnostic> {
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
        .with_message(err.message());

    codespan_to_lsp_diagnostics(diagnostic, location, url, src)
}

pub fn validation_error_to_codespan_diagnostic(
    err: &naga::WithSpan<naga::valid::ValidationError>,
    src: &str,
    module: &naga::Module,
) -> codespan_reporting::diagnostic::Diagnostic<()> {
    let ctx = ErrorContext::new(module, src);

    ctx.validation_error_diagnostic(err)
}

pub fn validation_error_to_lsp_diagnostic(
    err: &naga::WithSpan<naga::valid::ValidationError>,
    src: &str,
    url: &lsp_types::Uri,
    module: &naga::Module,
) -> Vec<lsp_types::Diagnostic> {
    codespan_to_lsp_diagnostics(
        validation_error_to_codespan_diagnostic(err, src, module),
        None,
        url,
        src,
    )
}
