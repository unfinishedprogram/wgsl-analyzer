use chumsky::span::SimpleSpan;
use std::fmt::{Debug, Formatter};

pub struct Spanned<T> {
    pub inner: T,
    pub span: SimpleSpan,
}

pub trait SpanAble: Sized {
    // Wraps the given value in a span
    fn with_span(self, span: SimpleSpan) -> Spanned<Self> {
        Spanned { inner: self, span }
    }

    // Copies the span from another spanned value
    fn with_span_of<T>(self, other: &dyn WithSpan<T>) -> Spanned<Self> {
        self.with_span(other.span())
    }
}

// Try to make span use as transparent as possible
impl<T> SpanAble for T where T: Sized {}
impl<T> Eq for Spanned<T> where T: Eq {}
impl<T> PartialEq for Spanned<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T> Copy for Spanned<T> where T: Copy {}
impl<T> Clone for Spanned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            span: self.span,
        }
    }
}

impl<T> Debug for Spanned<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:#?})", self.span, self.inner)
    }
}

pub trait WithSpan<T> {
    fn span(&self) -> SimpleSpan;
    fn as_inner(&self) -> &T;
    fn inner(self) -> T;
}

impl<T> WithSpan<T> for Spanned<T> {
    fn span(&self) -> SimpleSpan {
        self.span
    }

    fn as_inner(&self) -> &T {
        &self.inner
    }

    fn inner(self) -> T {
        self.inner
    }
}
