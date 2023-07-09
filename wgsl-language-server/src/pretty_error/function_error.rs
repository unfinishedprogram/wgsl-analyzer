use codespan_reporting::diagnostic::{Label, LabelStyle};
use naga::valid::FunctionError;

use super::{
    error_context::{span_priovider::SpanProvider, ContextErrorLabel},
    label_tools::secondary,
};

impl ContextErrorLabel for FunctionError {
    fn get_label(
        &self,
        context: &super::error_context::LabelContext,
        mut labels: Vec<Label<()>>,
    ) -> Vec<Label<()>> {
        match self {
            FunctionError::Expression { handle, source } => {
                labels.push(secondary("Invalid expression", context.get_span(*handle)));
                source.get_label(context, labels)
            }
            FunctionError::InvalidReturnType(expression) => {
                labels.push(secondary(
                    "Invalid return type",
                    context.get_span(context.function),
                ));

                let expected_type = context[context.function]
                    .result
                    .as_ref()
                    .and_then(|res| context[res.ty].name.to_owned())
                    .unwrap_or("void".into());

                let recieved_type = expression.map(|expr| &context[expr]);

                // if let Some(expression) =  {

                // } else {

                // }
                todo!();
            }
            _ => labels,
        }
    }
}
