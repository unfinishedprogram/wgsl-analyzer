use codespan_reporting::diagnostic::Diagnostic;
use naga::{valid::ValidationError, WithSpan};

use super::error_context::{ContextDiagnostic, ContextErrorLabel, ErrorContext, LabelContext};

impl ContextDiagnostic for WithSpan<ValidationError> {
    fn get_diagnostic(
        &self,
        context: &ErrorContext,
    ) -> codespan_reporting::diagnostic::Diagnostic<()> {
        match self.as_inner() {
            ValidationError::Function {
                handle,
                name,
                source,
            } => {
                let diagnostic =
                    Diagnostic::error().with_message(format!("function: '{}' is invalid", name));

                diagnostic
                    .with_labels(source.get_label(&LabelContext::new(context, *handle), vec![]))
            }

            _ => self.diagnostic(),
        }
    }
}
