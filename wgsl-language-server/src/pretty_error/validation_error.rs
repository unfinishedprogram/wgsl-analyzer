use naga::{valid::ValidationError, WithSpan};

use super::error_context::{ContextDiagnostic, ErrorContext};

impl ContextDiagnostic for WithSpan<ValidationError> {
    fn get_diagnostic(
        &self,
        context: &ErrorContext,
    ) -> codespan_reporting::diagnostic::Diagnostic<()> {
        self.diagnostic()
    }
}
