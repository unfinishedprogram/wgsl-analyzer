pub mod as_type;
pub mod index_impl;
pub mod type_print;

use as_type::AsType;
use codespan_reporting::diagnostic::Diagnostic;
use naga::{
    valid::{ExpressionError, FunctionError, ValidationError},
    Expression, Function, Handle, Module, Statement, WithSpan,
};
use type_print::TypePrintable;

use super::label_tools::{label_primary, label_secondary, LabelAppend};

pub struct ErrorContext<'a> {
    pub module: &'a Module,
    pub code: &'a str,
}

macro_rules! label {
    ($span:expr, $($arg:tt)*) => {
        label_secondary($span, format!($($arg)*))
    }
}

macro_rules! label_primary {
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
                name: _,
                source,
            } => self.function_error_diagnostic(Diagnostic::error(), *handle, source),
            other => {
                Diagnostic::error().with_message("UNIMPLEMENTED: ".to_string() + &other.to_string())
            }
        }
    }

    fn function_err_ctx(&self, function_handle: Handle<Function>) -> FunctionErrorContext {
        FunctionErrorContext {
            module: self.module,
            code: self.code,
            function: function_handle,
        }
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
            .with_label(label_primary!(
                &self.module.functions.get_span(handle),
                "Expected function `{}` to return type `{}`",
                func.name.clone().unwrap_or_default(),
                func.result
                    .as_ref()
                    .unwrap()
                    .ty
                    .print_type(&self.function_err_ctx(handle)),
            )),
            FunctionError::InvalidArgumentType { index: _, name: _ } => {
                diagnostic.with_label(label!(
                    &self.module.functions.get_span(handle),
                    "{}",
                    error.to_string()
                ))
            }
            FunctionError::InvalidIfType(expr_handle) => {
                let expr_type = self.type_of_expression_str(handle, *expr_handle);
                diagnostic
                    .with_label(label_primary!(
                        &func.expressions.get_span(*expr_handle),
                        "`if` condition must resolve to a scalar boolean value",
                    ))
                    .with_label(label!(
                        &func.expressions.get_span(*expr_handle),
                        "Expression of type `{}` found",
                        expr_type
                    ))
            }
            FunctionError::Expression {
                handle: expr_handle,
                source,
            } => self.expression_error_diagnostic(diagnostic, handle, *expr_handle, source),
            // FunctionError::ExpressionAlreadyInScope(_) => todo!(),
            // FunctionError::LocalVariable {
            //     handle,
            //     name,
            //     source,
            // } => todo!(),
            // FunctionError::NonConstructibleReturnType => todo!(),
            // FunctionError::InvalidArgumentPointerSpace { index, name, space } => todo!(),
            FunctionError::InstructionsAfterReturn => {
                let return_span = func
                    .body
                    .span_iter()
                    .find(|(stmt, _)| matches!(stmt, Statement::Return { .. }))
                    .map(|(_, span)| *span)
                    .unwrap_or_else(|| self.module.functions.get_span(handle));

                diagnostic.with_label(label!(&return_span, "{:?}", &error))
            }
            // FunctionError::BreakOutsideOfLoopOrSwitch => todo!(),
            // FunctionError::ContinueOutsideOfLoop => todo!(),
            // FunctionError::InvalidReturnSpot => todo!(),

            // FunctionError::InvalidSwitchType(_) => todo!(),
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
            _ => diagnostic.with_label(label!(
                &self.module.functions.get_span(handle),
                "{:?}",
                &error
            )),
        }
    }

    fn expression_error_diagnostic(
        &self,
        diagnostic: Diagnostic<()>,
        function_handle: Handle<Function>,
        expr_handle: Handle<Expression>,
        error: &ExpressionError,
    ) -> Diagnostic<()> {
        let func = &self.module.functions[function_handle];
        let expr_span = func.expressions.get_span(expr_handle);
        self.type_of_expression_str(function_handle, expr_handle);

        match error {
            ExpressionError::NotInScope => {
                diagnostic.with_label(label!(&expr_span, "{}", error.to_string()))
            }
            ExpressionError::InvalidBaseType(handle) => diagnostic.with_label(label!(
                &expr_span,
                "Accessing with index {:} of type {:} can't be done",
                self.code_in_fn(function_handle, expr_handle),
                self.type_of_expression_str(function_handle, *handle)
            )),
            // ExpressionError::InvalidIndexType(handle) => todo!(),
            // ExpressionError::NegativeIndex(handle) => todo!(),
            // ExpressionError::IndexOutOfBounds(handle, _) => todo!(),
            // ExpressionError::IndexMustBeConstant(handle) => todo!(),
            // ExpressionError::FunctionArgumentDoesntExist(_) => todo!(),
            // ExpressionError::InvalidPointerType(handle) => todo!(),
            // ExpressionError::InvalidArrayType(handle) => todo!(),
            // ExpressionError::InvalidRayQueryType(handle) => todo!(),
            // ExpressionError::InvalidSplatType(handle) => todo!(),
            // ExpressionError::InvalidVectorType(handle) => todo!(),
            // ExpressionError::InvalidSwizzleComponent(swizzle_component, vector_size) => todo!(),
            // ExpressionError::Compose(compose_error) => todo!(),
            // ExpressionError::IndexableLength(indexable_length_error) => todo!(),
            // ExpressionError::InvalidUnaryOperandType(unary_operator, handle) => todo!(),
            ExpressionError::InvalidBinaryOperandTypes(binary_operator, handle_a, handle_b) => {
                let type_a = self.type_of_expression_str(function_handle, *handle_a);
                let type_b = self.type_of_expression_str(function_handle, *handle_b);

                diagnostic.with_label(label_primary!(
                    &expr_span,
                    "Operation {binary_operator:?} can't work with types {type_a:} and {type_b:}",
                ))
            }

            // ExpressionError::InvalidSelectTypes => todo!(),
            // ExpressionError::InvalidBooleanVector(handle) => todo!(),
            // ExpressionError::InvalidFloatArgument(handle) => todo!(),
            // ExpressionError::Type(resolve_error) => todo!(),
            // ExpressionError::ExpectedGlobalVariable => todo!(),
            // ExpressionError::ExpectedGlobalOrArgument => todo!(),
            // ExpressionError::ExpectedBindingArrayType(handle) => todo!(),
            // ExpressionError::ExpectedImageType(handle) => todo!(),
            // ExpressionError::ExpectedSamplerType(handle) => todo!(),
            // ExpressionError::InvalidImageClass(image_class) => todo!(),
            // ExpressionError::InvalidDerivative => todo!(),
            // ExpressionError::InvalidImageArrayIndex => todo!(),
            // ExpressionError::InvalidImageOtherIndex => todo!(),
            // ExpressionError::InvalidImageArrayIndexType(handle) => todo!(),
            // ExpressionError::InvalidImageOtherIndexType(handle) => todo!(),
            // ExpressionError::InvalidImageCoordinateType(image_dimension, handle) => todo!(),
            // ExpressionError::ComparisonSamplingMismatch {
            //     image,
            //     sampler,
            //     has_ref,
            // } => todo!(),
            // ExpressionError::InvalidSampleOffsetExprType => todo!(),
            // ExpressionError::InvalidSampleOffset(image_dimension, handle) => todo!(),
            // ExpressionError::InvalidDepthReference(handle) => todo!(),
            // ExpressionError::InvalidDepthSampleLevel => todo!(),
            // ExpressionError::InvalidGatherLevel => todo!(),
            // ExpressionError::InvalidGatherComponent(swizzle_component) => todo!(),
            // ExpressionError::InvalidGatherDimension(image_dimension) => todo!(),
            // ExpressionError::InvalidSampleLevelExactType(handle) => todo!(),
            // ExpressionError::InvalidSampleLevelBiasType(handle) => todo!(),
            // ExpressionError::InvalidSampleLevelGradientType(image_dimension, handle) => todo!(),
            // ExpressionError::InvalidCastArgument => todo!(),
            // ExpressionError::WrongArgumentCount(math_function) => todo!(),
            // ExpressionError::InvalidArgumentType(math_function, _, handle) => todo!(),
            // ExpressionError::InvalidWorkGroupUniformLoadResultType(handle) => todo!(),
            // ExpressionError::MissingCapabilities(capabilities) => todo!(),
            // ExpressionError::Literal(literal_error) => todo!(),
            // ExpressionError::UnsupportedWidth(math_function, scalar_kind, _) => todo!(),
            _ => diagnostic.with_label(label!(&expr_span, "{}", error.to_string())),
        }
    }

    fn code_in_fn(
        &self,
        function_handle: Handle<Function>,
        expr_handle: Handle<Expression>,
    ) -> &str {
        let func = &self.module.functions[function_handle];
        let span = func.expressions.get_span(expr_handle);
        &self.code[span]
    }
}

pub struct FunctionErrorContext<'a> {
    pub module: &'a Module,
    #[allow(unused)]
    pub code: &'a str,
    pub function: Handle<Function>,
}
