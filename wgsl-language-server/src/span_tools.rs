use codespan_reporting::diagnostic::{Label, LabelStyle};
use naga::{Span, WithSpan};

pub trait SpanTools {
    fn total_span(&self) -> Span;
}

pub fn spans_as_labels(spans: Vec<(Span, String)>) -> Vec<Label<()>> {
    spans
        .into_iter()
        .map(|(span, message)| Label {
            style: LabelStyle::Primary,
            file_id: (),
            range: span.to_range().unwrap(),
            message,
        })
        .collect()
}

impl<T> SpanTools for WithSpan<T> {
    fn total_span(&self) -> Span {
        let mut span: Span = Default::default();
        for other in self.spans().map(|(span, _)| span) {
            span.subsume(*other);
        }
        span
    }
}
