use naga::{Span, WithSpan};

pub trait SpanTools {
    fn total_span(&self) -> Span;
}

impl<T> SpanTools for WithSpan<T> {
    fn total_span(&self) -> Span {
        let mut span: Span = Default::default();
        for other in self.spans().map(|(span, context)| span) {
            span.subsume(*other);
        }
        span
    }
}
