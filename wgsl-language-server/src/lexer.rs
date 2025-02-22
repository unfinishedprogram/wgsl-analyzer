use std::ops::Range;

use comment::lex_multiline_comment;
use logos::Logos;
mod comment;
mod keyword;
mod template_disambiguation;
mod test;
use keyword::{parse_ident, IdentError, Keyword};
use template_disambiguation::insert_template_tokens;

use crate::log;

// TODO: Cleanup error filtering
pub fn lex(src: &str) -> Option<Vec<(Token<'_>, Range<usize>)>> {
    let tokens = Token::lexer(src).spanned().collect::<Vec<_>>();

    if tokens.iter().any(|it| it.0.is_err()) {
        let span = tokens.iter().find(|it| it.0.is_err()).unwrap().1.clone();

        log!(
            "Failed to lex source code: {:?} \n {:?}",
            tokens.iter().find(|it| it.0.is_err()),
            &src[span]
        );
        return None;
    };

    let mut tokens = tokens
        .iter()
        .map(|(tok, span)| (tok.clone().unwrap(), span.clone()))
        .collect::<Vec<_>>();

    insert_template_tokens(src, &mut tokens);

    Some(tokens)
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LexError {
    InvalidIdentifier(IdentError),
    UnterminatedComment,
    #[default]
    Other,
}

impl From<IdentError> for LexError {
    fn from(err: IdentError) -> Self {
        LexError::InvalidIdentifier(err)
    }
}

#[derive(Logos, Debug, PartialEq, Eq, Clone)]
#[logos(error = LexError)]
#[logos(skip r"\s")]
pub enum Token<'src> {
    #[token("alias", |_| Keyword::Alias)]
    #[token("break", |_| Keyword::Break)]
    #[token("case", |_| Keyword::Case)]
    #[token("const", |_| Keyword::Const)]
    #[token("const_assert", |_| Keyword::ConstAssert)]
    #[token("continue", |_| Keyword::Continue)]
    #[token("continuing", |_| Keyword::Continuing)]
    #[token("default", |_| Keyword::Default)]
    #[token("diagnostic", |_| Keyword::Diagnostic)]
    #[token("discard", |_| Keyword::Discard)]
    #[token("else", |_| Keyword::Else)]
    #[token("enable", |_| Keyword::Enable)]
    #[token("fn", |_| Keyword::Fn)]
    #[token("for", |_| Keyword::For)]
    #[token("if", |_| Keyword::If)]
    #[token("let", |_| Keyword::Let)]
    #[token("loop", |_| Keyword::Loop)]
    #[token("override", |_| Keyword::Override)]
    #[token("requires", |_| Keyword::Requires)]
    #[token("return", |_| Keyword::Return)]
    #[token("struct", |_| Keyword::Struct)]
    #[token("switch", |_| Keyword::Switch)]
    #[token("var", |_| Keyword::Var)]
    #[token("while", |_| Keyword::While)]
    Keyword(Keyword),

    #[token("<<=")]
    #[token(">>=")]
    #[token("==")]
    #[token("!=")]
    #[token("<=")]
    #[token(">=")]
    #[token("&&")]
    #[token("||")]
    #[token("->")]
    #[token("=>")]
    #[token("++")]
    #[token("--")]
    #[token("+=")]
    #[token("-=")]
    #[token("*=")]
    #[token("/=")]
    #[token("%=")]
    #[token("&=")]
    #[token("|=")]
    #[token("^=")]
    #[token(">>")]
    #[token("<<")]
    #[token("(")]
    #[token(")")]
    #[token("[")]
    #[token("]")]
    #[token("{")]
    #[token("}")]
    #[token(";")]
    #[token(".")]
    #[token(",")]
    #[token(":")]
    #[token("&")]
    #[token("|")]
    #[token("^")]
    #[token("@")]
    #[token("=")]
    #[token(">")]
    #[token("<")]
    #[token("%")]
    #[token("/")]
    #[token("+")]
    #[token("-")]
    #[token("*")]
    #[token("~")]
    #[token("!")]
    Syntax(&'src str),

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    Boolean(bool),

    #[regex(r"([_\p{XID_Start}][\p{XID_Continue}]+)|([\p{XID_Start}])|_", |lex| parse_ident(lex.slice()), priority = 2)]
    Ident(&'src str),

    #[regex(r"0[iu]?")] // Zero Values
    #[regex(r"[1-9][0-9]*[iu]?")] // Decimal Literals
    #[regex(r"0[xX][0-9a-fA-F]+[iu]?")] // Hex Literals
    Integer(&'src str),

    // Decimal Float Literals
    #[regex(r"0[fh]")]
    #[regex(r"[1-9][0-9]*[fh]")]
    #[regex(r"[0-9]*\.[0-9]+([eE][+-]?[0-9]+)?[fh]?")]
    #[regex(r"[0-9]+\.[0-9]*([eE][+-]?[0-9]+)?[fh]?", priority = 5)]
    #[regex(r"[0-9]+[eE][+-]?[0-9]+[fh]?")]
    // Hex Float Literals
    #[regex(
        r"0[xX][0-9a-fA-F]*\.[0-9a-fA-F]+([pP][+-]?[0-9]+[fh]?)?",
        priority = 9
    )]
    #[regex(r"0[xX][0-9a-fA-F]+\.[0-9a-fA-F]*([pP][+-]?[0-9]+[fh]?)?")]
    #[regex(r"0[xX][0-9a-fA-F]+[pP][+-]?[0-9]+[fh]?")]
    Float(&'src str),

    #[regex(r"/\*", lex_multiline_comment)]
    #[regex(r"\/\/.*\n")]
    Trivia(&'src str),

    TemplateArgsStart,
    TemplateArgsEnd,
}
