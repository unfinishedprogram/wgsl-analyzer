use naga::ScalarValue;

use super::error_context::ContextErrorFmt;

impl ContextErrorFmt for ScalarValue {
    fn print(&self, _: &super::error_context::LabelContext) -> String {
        match self {
            ScalarValue::Sint(value) => format!("{value}"),
            ScalarValue::Uint(value) => format!("{value}"),
            ScalarValue::Float(value) => format!("{value}"),
            ScalarValue::Bool(value) => if *value { "True" } else { "False" }.into(),
        }
    }
}
