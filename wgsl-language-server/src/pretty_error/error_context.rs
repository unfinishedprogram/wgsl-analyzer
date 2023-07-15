pub mod as_type;
pub mod code_provider;
pub mod index_impl;
pub mod span_priovider;
pub mod type_print;

use codespan_reporting::diagnostic::Label;
use naga::{Function, Handle, Module};

use super::label_tools::AsRange;

pub struct ErrorContext<'a> {
    pub module: &'a Module,
    pub code: &'a str,
}

pub trait ContextDiagnostic {
    fn get_diagnostic(
        &self,
        context: &ErrorContext,
    ) -> codespan_reporting::diagnostic::Diagnostic<()>;
}

impl<'a> ErrorContext<'a> {
    pub fn new(module: &'a Module, code: &'a str) -> Self {
        Self { module, code }
    }

    pub fn get_diagnostic(
        &self,
        err: &dyn ContextDiagnostic,
    ) -> codespan_reporting::diagnostic::Diagnostic<()> {
        err.get_diagnostic(self)
    }
}

pub struct LabelContext<'a> {
    pub error_context: &'a ErrorContext<'a>,
    pub function: Handle<Function>,
}

impl<'a> LabelContext<'a> {
    pub fn new(error_context: &'a ErrorContext, function: Handle<Function>) -> Self {
        Self {
            error_context,
            function,
        }
    }
}

pub fn append_info(mut labels: Vec<Label<()>>, message: impl Into<String>) -> Vec<Label<()>> {
    let range = labels.last().map(|v| v.range.clone()).unwrap_or_default();
    labels.push(Label::secondary((), range).with_message(message.into()));
    labels
}

pub fn append_primary(
    mut labels: Vec<Label<()>>,
    message: impl Into<String>,
    range: impl AsRange,
) -> Vec<Label<()>> {
    let range = range.as_range();
    labels.push(Label::primary((), range).with_message(message.into()));
    labels
}

pub trait ContextErrorLabel {
    fn get_label(&self, context: &LabelContext, labels: Vec<Label<()>>) -> Vec<Label<()>>;
}

pub trait ContextErrorPrint {
    fn print(&self, context: &LabelContext) -> String;
}
