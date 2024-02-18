use chumsky::span::SimpleSpan;

pub enum VariableValue {
    Variable(Variable),
    Value(Value),
}

pub struct Variable {
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
