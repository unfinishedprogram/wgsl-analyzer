use crate::range_tools::lsp_range_from_char_span;

pub fn wgsl_error_to_lsp_diagnostic(
    uri: lsp_types::Url,
    source: &str,
    diagnostic: &wgsl_ast::diagnostic::Diagnostic,
) -> lsp_types::Diagnostic {
    let span = diagnostic.span.map(|span| span.into()).unwrap_or(0..0);
    let range = lsp_range_from_char_span(source, &span);
    let message = diagnostic.message.clone();

    let related_information: Vec<lsp_types::DiagnosticRelatedInformation> = diagnostic
        .related_info
        .iter()
        .map(|info| {
            let range = lsp_range_from_char_span(source, &info.span.into());
            lsp_types::DiagnosticRelatedInformation {
                location: lsp_types::Location {
                    uri: uri.clone(),
                    range,
                },
                message: info.message.clone(),
            }
        })
        .collect();

    lsp_types::Diagnostic {
        message,
        range,
        related_information: Some(related_information),
        source: Some("wgsl-language-support".to_owned()),
        ..Default::default()
    }
}
