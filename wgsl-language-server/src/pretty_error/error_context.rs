pub mod as_type;
pub mod code_provider;
pub mod index_impl;
pub mod span_priovider;
pub mod type_print;

use as_type::AsType;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use naga::{
    valid::{FunctionError, ValidationError},
    Expression, Function, Handle, Module, Span, WithSpan,
};
use type_print::TypePrintable;

use super::label_tools::{AsRange, LabelAppend};

pub struct ErrorContext<'a> {
    pub module: &'a Module,
    pub code: &'a str,
}

fn label_primary(span: &impl AsRange, msg: impl Into<String>) -> Label<()> {
    Label::primary((), span.get_range()).with_message(msg)
}

impl<'a> ErrorContext<'a> {
    pub fn new(module: &'a Module, code: &'a str) -> Self {
        Self { module, code }
    }

    pub fn validation_error_diagnostic(&self, error: &WithSpan<ValidationError>) -> Diagnostic<()> {
        match error.as_inner() {
            ValidationError::Function {
                handle,
                name,
                source,
            } => self.function_error_diagnostic(
                Diagnostic::error()
                    .with_label(label_primary(error, format!("Function {name} is invalid"))),
                *handle,
                source,
            ),
            other => {
                Diagnostic::error().with_message("UNIMPLEMENTED: ".to_string() + &other.to_string())
            }
        }
    }

    fn label_ctx(&self, function_handle: Handle<Function>) -> LabelContext {
        LabelContext::new(self, function_handle)
    }

    fn type_of_expression_str(
        &self,
        function_handle: Handle<Function>,
        expr_handle: Handle<Expression>,
    ) -> String {
        let label_ctx = self.label_ctx(function_handle);
        expr_handle.as_type(&label_ctx).print_type(&label_ctx)
    }

    fn function_error_diagnostic(
        &self,
        diagnostic: Diagnostic<()>,
        handle: Handle<Function>,
        error: &FunctionError,
    ) -> Diagnostic<()> {
        let func = &self.module.functions[handle];
        match error {
            FunctionError::InvalidReturnType(return_expr) => if let Some(expr_handle) = return_expr
            {
                let return_span = func.expressions.get_span(*expr_handle);
                diagnostic.with_label(label_primary(
                    &return_span,
                    format!(
                        "Expression of type {} returned",
                        self.type_of_expression_str(handle, *expr_handle)
                    ),
                ))
            } else {
                diagnostic
            }
            .with_message(format!(
                "Expected function {} to return {}",
                func.name.clone().unwrap_or_default(),
                func.result
                    .as_ref()
                    .unwrap()
                    .ty
                    .print_type(&self.label_ctx(handle)),
            )),
            _ => diagnostic.with_message("UNIMPLEMENTED".to_string() + &error.to_string()),
            // FunctionError::Expression { handle, source } => todo!(),
            // FunctionError::ExpressionAlreadyInScope(_) => todo!(),
            // FunctionError::LocalVariable {
            //     handle,
            //     name,
            //     source,
            // } => todo!(),
            // FunctionError::InvalidArgumentType { index, name } => todo!(),
            // FunctionError::NonConstructibleReturnType => todo!(),
            // FunctionError::InvalidArgumentPointerSpace { index, name, space } => todo!(),
            // FunctionError::InstructionsAfterReturn => todo!(),
            // FunctionError::BreakOutsideOfLoopOrSwitch => todo!(),
            // FunctionError::ContinueOutsideOfLoop => todo!(),
            // FunctionError::InvalidReturnSpot => todo!(),

            // FunctionError::InvalidIfType(_) => todo!(),
            // FunctionError::InvalidSwitchType(_) => todo!(),
            // FunctionError::ConflictingSwitchCase(_) => todo!(),
            // FunctionError::ConflictingCaseType => todo!(),
            // FunctionError::MissingDefaultCase => todo!(),
            // FunctionError::MultipleDefaultCases => todo!(),
            // FunctionError::LastCaseFallTrough => todo!(),
            // FunctionError::InvalidStorePointer(_) => todo!(),
            // FunctionError::InvalidStoreValue(_) => todo!(),
            // FunctionError::InvalidStoreTypes { pointer, value } => todo!(),
            // FunctionError::InvalidImageStore(_) => todo!(),
            // FunctionError::InvalidCall { function, error } => todo!(),
            // FunctionError::InvalidAtomic(_) => todo!(),
            // FunctionError::InvalidRayQueryExpression(_) => todo!(),
            // FunctionError::InvalidAccelerationStructure(_) => todo!(),
            // FunctionError::InvalidRayDescriptor(_) => todo!(),
            // FunctionError::InvalidRayQueryType(_) => todo!(),
            // FunctionError::MissingCapability(_) => todo!(),
            // FunctionError::NonUniformControlFlow(_, _, _) => todo!(),
            // FunctionError::PipelineInputRegularFunction { name } => todo!(),
            // FunctionError::PipelineOutputRegularFunction => todo!(),
            // FunctionError::NonUniformWorkgroupUniformLoad(_) => todo!(),
            // FunctionError::WorkgroupUniformLoadExpressionMismatch(_) => todo!(),
            // FunctionError::WorkgroupUniformLoadInvalidPointer(_) => todo!(),
            // FunctionError::InvalidSubgroup(_) => todo!(),
            // FunctionError::EmitResult(_) => todo!(),
            // FunctionError::UnvisitedExpression(_) => todo!(),
        }
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
    let range = range.get_range();
    labels.push(Label::primary((), range).with_message(message.into()));
    labels
}

pub trait ContextErrorLabel {
    fn get_label(&self, context: &LabelContext, labels: Vec<Label<()>>) -> Vec<Label<()>>;
}

pub trait ContextErrorFmt {
    fn print(&self, context: &LabelContext) -> String;
}
