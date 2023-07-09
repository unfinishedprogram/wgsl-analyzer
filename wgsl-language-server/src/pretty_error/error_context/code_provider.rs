use naga::Handle;

use crate::pretty_error::label_tools::AsRange;

use super::{span_priovider::SpanProvider, ErrorContext, LabelContext};

pub trait CodeProvider<T>: SpanProvider<T> {
    fn code(&self) -> &str;
    fn code_span(&self, handle: Handle<T>) -> &str {
        &self.code()[self.get_span(handle).as_range()]
    }
}

impl<'a, T> CodeProvider<T> for LabelContext<'a>
where
    LabelContext<'a>: SpanProvider<T>,
{
    fn code(&self) -> &str {
        self.error_context.code
    }
}

impl<'a, T> CodeProvider<T> for ErrorContext<'a>
where
    ErrorContext<'a>: SpanProvider<T>,
{
    fn code(&self) -> &str {
        self.code
    }
}
