use chumsky::{error::Rich, extra, span::SimpleSpan};

mod ident;
pub use ident::ident;

pub mod parse;
pub mod template;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    Boolean(bool),
    Int(String),
    Float(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'src> {
    Literal(Literal),
    Keyword(Keyword),
    SyntaxToken(&'src str),
    Ident(&'src str),
    Trivia,
    TemplateArgsStart,
    TemplateArgsEnd,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Keyword {
    Alias,
    Break,
    Case,
    Const,
    ConstAssert,
    Continue,
    Continuing,
    Default,
    Diagnostic,
    Discard,
    Else,
    Enable,
    Fn,
    For,
    If,
    Let,
    Loop,
    Override,
    Requires,
    Return,
    Struct,
    Switch,
    Var,
    While,
}

impl From<Keyword> for &'static str {
    fn from(val: Keyword) -> Self {
        match val {
            Keyword::Alias => "alias",
            Keyword::Break => "break",
            Keyword::Case => "case",
            Keyword::Const => "const",
            Keyword::ConstAssert => "const_assert",
            Keyword::Continue => "continue",
            Keyword::Continuing => "continuing",
            Keyword::Default => "default",
            Keyword::Diagnostic => "diagnostic",
            Keyword::Discard => "discard",
            Keyword::Else => "else",
            Keyword::Enable => "enable",
            Keyword::Fn => "fn",
            Keyword::For => "for",
            Keyword::If => "if",
            Keyword::Let => "let",
            Keyword::Loop => "loop",
            Keyword::Override => "override",
            Keyword::Requires => "requires",
            Keyword::Return => "return",
            Keyword::Struct => "struct",
            Keyword::Switch => "switch",
            Keyword::Var => "var",
            Keyword::While => "while",
        }
    }
}

impl From<Keyword> for String {
    fn from(val: Keyword) -> Self {
        Into::<&str>::into(val).to_owned()
    }
}

// A rich error type only for the tokenization step
type RichErr<'src> = extra::Err<Rich<'src, char, SimpleSpan>>;
