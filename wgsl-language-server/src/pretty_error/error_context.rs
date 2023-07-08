use naga::Module;

pub struct ErrorContext<'a> {
    pub module: &'a Module,
    pub code: &'a str,
}

pub trait ContextDiagnostic {
    fn get_diagnostic(
        &self,
        context: &ErrorContext,
    ) -> codespan_reporting::diagnostic::Diagnostic<()>;
}

pub trait ContextErrorLabel {
    fn get_label(&self, context: ErrorContext) -> String;
}

impl<'a> ErrorContext<'a> {
    pub fn new(module: &'a Module, code: &'a str) -> Self {
        Self { module, code }
    }

    pub fn get_diagnostic(
        &self,
        err: &dyn ContextDiagnostic,
    ) -> codespan_reporting::diagnostic::Diagnostic<()> {
        err.get_diagnostic(self)
    }
}
