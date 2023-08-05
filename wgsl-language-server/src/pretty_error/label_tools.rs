// Tools for simplifying the construction of codespan labels

use std::ops::Range;

pub trait AsRange {
    fn as_range(&self) -> Range<usize>;
}

impl AsRange for naga::Span {
    fn as_range(&self) -> Range<usize> {
        self.to_range().unwrap_or_default()
    }
}
