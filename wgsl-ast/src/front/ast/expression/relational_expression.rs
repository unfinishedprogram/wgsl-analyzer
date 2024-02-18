#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiplicativeOperator {
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdditiveOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiftOperator {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    Negative,
    Not,
    BitNot,
    Deref,
    AddrOf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortCircuitOperator {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitwiseOperator {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalOperator {
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    Multiplicative(MultiplicativeOperator),
    Additive(AdditiveOperator),
    Shift(ShiftOperator),
    ShortCircuit(ShortCircuitOperator),
    Bitwise(BitwiseOperator),
    Relational(RelationalOperator),
}
