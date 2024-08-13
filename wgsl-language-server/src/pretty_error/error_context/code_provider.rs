use naga::Handle;

use crate::pretty_error::label_tools::AsRange;

use super::{span_provider::SpanProvider, ErrorContext, FunctionErrorContext};

pub trait CodeProvider<T>: SpanProvider<T> {
    fn code(&self) -> &str;
    fn code_span(&self, handle: Handle<T>) -> &str {
        &self.code()[self.get_span(handle).get_range()]
    }
}

impl<'a, T> CodeProvider<T> for FunctionErrorContext<'a>
where
    FunctionErrorContext<'a>: SpanProvider<T>,
{
    fn code(&self) -> &str {
        self.code
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
