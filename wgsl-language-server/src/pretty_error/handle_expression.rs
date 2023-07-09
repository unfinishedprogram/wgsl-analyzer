use naga::{Expression, Handle};

use super::error_context::{ContextErrorPrint, LabelContext};

impl ContextErrorPrint for Handle<Expression> {
    fn print(&self, context: &LabelContext) -> String {
        let expression_range = context.error_context.module.functions[context.function]
            .expressions
            .get_span(*self)
            .to_range()
            .unwrap_or_default();
        context.error_context.code[expression_range].into()
    }
}
