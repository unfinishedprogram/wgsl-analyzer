use codespan_reporting::diagnostic::Label;
use naga::valid::ExpressionError;

use super::error_context::{append_info, ContextErrorFmt, ContextErrorLabel};

impl ContextErrorLabel for ExpressionError {
    fn get_label(
        &self,
        context: &super::error_context::LabelContext,
        labels: Vec<Label<()>>,
    ) -> Vec<Label<()>> {
        match self {
            ExpressionError::DoesntExist => append_info(labels, "Expression does not exist"),
            ExpressionError::NotInScope => append_info(labels, "Expression is out of scope"),
            ExpressionError::IndexOutOfBounds(expression, scalar) => {
                let scalar = scalar.print(context);
                let expression = expression.print(context);
                append_info(
                    labels,
                    format!("Index: '{scalar}' is out of bounds for '{expression}'"),
                )
            }
            _ => labels,
        }
    }
}
