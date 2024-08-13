use naga::{Expression, Handle};

use super::error_context::{code_provider::CodeProvider, ContextErrorFmt, FunctionErrorContext};

impl ContextErrorFmt for Handle<Expression> {
    fn print(&self, context: &FunctionErrorContext) -> String {
        context.code_span(*self).into()
    }
}
