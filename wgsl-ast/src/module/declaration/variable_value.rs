use chumsky::span::SimpleSpan;

use crate::front::ast::statement::declaration;

pub enum VariableValue {
    Variable(Variable),
    Value(Value),
}

pub struct Variable {
    ast: declaration::Variable,
    span: SimpleSpan,
}

pub enum ValueKind {
    Const,
    Override,
    Let,
    FormalParameter,
}

pub struct Value {
    span: SimpleSpan,
    kind: ValueKind,
}
