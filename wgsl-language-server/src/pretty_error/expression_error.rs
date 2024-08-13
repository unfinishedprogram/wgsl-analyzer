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
            ExpressionError::NotInScope => append_info(labels, "Expression is out of scope"),
            ExpressionError::IndexOutOfBounds(expression, index) => {
                let expression = expression.print(context);
                append_info(
                    labels,
                    format!("Index: '{index}' is out of bounds for '{expression}'"),
                )
            }
            _ => labels,
        }
    }
}
