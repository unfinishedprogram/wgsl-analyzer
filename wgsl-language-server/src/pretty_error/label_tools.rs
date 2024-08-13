// Tools for simplifying the construction of codespan labels

use std::ops::Range;

use codespan_reporting::diagnostic::Label;
use naga::WithSpan;

pub trait AsRange {
    fn get_range(&self) -> Range<usize>;
}

impl AsRange for naga::Span {
    fn get_range(&self) -> Range<usize> {
        self.to_range().unwrap_or_default()
    }
}

impl<T> AsRange for WithSpan<T> {
    fn get_range(&self) -> Range<usize> {
        self.spans()
            .next()
            .map(|(span, _)| span.get_range())
            .unwrap_or_default()
    }
}

// A helper trait to allow for easily appending labels to Diagnostics
// Otherwise, each call to with_labels would need to be wrapped in a vec![]
pub trait LabelAppend {
    fn with_label(self, label: Label<()>) -> Self;
}

impl LabelAppend for codespan_reporting::diagnostic::Diagnostic<()> {
    fn with_label(self, label: Label<()>) -> Self {
        self.with_labels(vec![label])
    }
}
