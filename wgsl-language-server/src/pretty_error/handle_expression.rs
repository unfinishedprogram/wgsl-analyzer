use naga::{Expression, Handle};

use super::error_context::{code_provider::CodeProvider, ContextErrorFmt, LabelContext};

impl ContextErrorFmt for Handle<Expression> {
    fn print(&self, context: &LabelContext) -> String {
        context.code_span(*self).into()
    }
}
