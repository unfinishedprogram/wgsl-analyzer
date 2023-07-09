use naga::{Expression, Function, Handle, Span};

use super::{ErrorContext, LabelContext};

pub trait SpanProvider<T> {
    fn get_span(&self, handle: Handle<T>) -> Span;
}

/*-----------
Error Context
-----------*/

impl SpanProvider<Function> for ErrorContext<'_> {
    fn get_span(&self, handle: Handle<Function>) -> Span {
        self.module.functions.get_span(handle)
    }
}

/*-----------
Label Context
-----------*/

impl SpanProvider<Function> for LabelContext<'_> {
    fn get_span(&self, handle: Handle<Function>) -> Span {
        self.error_context.get_span(handle)
    }
}

impl SpanProvider<Expression> for LabelContext<'_> {
    fn get_span(&self, handle: Handle<Expression>) -> Span {
        self[self.function].expressions.get_span(handle)
    }
}
