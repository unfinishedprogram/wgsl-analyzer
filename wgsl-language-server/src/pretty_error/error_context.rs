pub mod as_type;
pub mod code_provider;
pub mod index_impl;
pub mod span_provider;
pub mod type_print;

use as_type::AsType;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use naga::{
    valid::{FunctionError, ValidationError},
    Expression, Function, Handle, Module, WithSpan,
};
use type_print::TypePrintable;

use super::label_tools::{label_primary, AsRange, LabelAppend};

pub struct ErrorContext<'a> {
    pub module: &'a Module,
    pub code: &'a str,
}

macro_rules! label {
    ($span:expr, $($arg:tt)*) => {
        label_primary($span, format!($($arg)*))
    }
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
                Diagnostic::error().with_label(label!(error, "Function {name} is invalid")),
                *handle,
                source,
            ),
            other => {
                Diagnostic::error().with_message("UNIMPLEMENTED: ".to_string() + &other.to_string())
            }
        }
    }

    fn function_err_ctx(&self, function_handle: Handle<Function>) -> FunctionErrorContext {
        FunctionErrorContext::new(self, function_handle)
    }

    fn type_of_expression_str(
        &self,
        function_handle: Handle<Function>,
        expr_handle: Handle<Expression>,
    ) -> String {
        let label_ctx = self.function_err_ctx(function_handle);
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
                diagnostic.with_label(label!(
                    &return_span,
                    "Expression of type `{}` returned",
                    self.type_of_expression_str(handle, *expr_handle)
                ))
            } else {
                diagnostic.with_label(label!(
                    &self.module.functions.get_span(handle),
                    "Function does not always return a value",
                ))
            }
            .with_label(label!(
                &self.module.functions.get_span(handle),
                "Expected function `{}` to return type `{}`",
                func.name.clone().unwrap_or_default(),
                func.result
                    .as_ref()
                    .unwrap()
                    .ty
                    .print_type(&self.function_err_ctx(handle)),
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

pub struct FunctionErrorContext<'a> {
    pub module: &'a Module,
    pub code: &'a str,
    pub function: Handle<Function>,
}

impl<'a> FunctionErrorContext<'a> {
    pub fn new(error_context: &'a ErrorContext, function: Handle<Function>) -> Self {
        Self {
            module: &error_context.module,
            code: error_context.code,
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

pub trait ContextErrorFmt {
    fn print(&self, context: &FunctionErrorContext) -> String;
}
