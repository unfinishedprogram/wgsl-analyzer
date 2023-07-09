use codespan_reporting::diagnostic::Label;
use naga::valid::FunctionError;

use crate::pretty_error::error_context::{
    as_type::AsType, type_print::TypePrintable, ContextErrorPrint,
};

use super::{
    error_context::{span_priovider::SpanProvider, ContextErrorLabel},
    label_tools::{primary, secondary},
};

impl ContextErrorLabel for FunctionError {
    fn get_label(
        &self,
        context: &super::error_context::LabelContext,
        mut labels: Vec<Label<()>>,
    ) -> Vec<Label<()>> {
        labels.push(secondary(
            "Invalid Function",
            context.get_span(context.function),
        ));

        match self {
            FunctionError::Expression { handle, source } => {
                labels.push(secondary("Invalid expression", context.get_span(*handle)));
                source.get_label(context, labels)
            }

            // TODO: Fix expected return type is always void
            FunctionError::InvalidReturnType(expression) => {
                let range = if let Some(expression) = expression {
                    context.get_span(*expression)
                } else {
                    context.get_span(context.function)
                };

                let expected_type = context[context.function]
                    .result
                    .as_ref()
                    .and_then(|res| context[res.ty].name.to_owned())
                    .unwrap_or("void".into());

                let message = if let Some(expr_handle) = expression {
                    let received_type = expr_handle.as_type(context).print_type(context);
                    format!(
                        "Expression: '{}' of type '{}', Does not match expected type '{}'",
                        expr_handle.print(context),
                        received_type,
                        expected_type
                    )
                } else {
                    format!("Expected return type '{expected_type}' received 'void'")
                };

                labels.push(primary(message, range));
                labels
            }
            _ => labels,
        }
    }
}
