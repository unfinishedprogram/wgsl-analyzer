// Tools for simplifying the construction of codespan labels

use std::ops::Range;

use codespan_reporting::diagnostic::{Label, LabelStyle};

pub trait AsRange {
    fn as_range(&self) -> Range<usize>;
}

impl AsRange for naga::Span {
    fn as_range(&self) -> Range<usize> {
        self.to_range().unwrap_or_default()
    }
}

pub fn secondary(message: impl Into<String>, range: impl AsRange) -> Label<()> {
    Label {
        style: LabelStyle::Secondary,
        file_id: (),
        range: range.as_range(),
        message: message.into(),
    }
}
