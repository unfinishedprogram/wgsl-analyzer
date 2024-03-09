pub mod expression;
pub mod statement;

use self::statement::{declaration::Declaration, statement, Statement};
use super::{
    span::{SpanAble, Spanned},
    token::{parse::tokenizer, template::insert_template_tokens, Token},
};
use chumsky::prelude::*;

type ParserInput<'tokens, 'src> = chumsky::input::SpannedInput<
    Token<'src>,
    SimpleSpan<usize>,
    &'tokens [(Token<'src>, SimpleSpan<usize>)],
>;

type RichErr<'src, 'tokens> = extra::Err<Rich<'tokens, Token<'src>, SimpleSpan>>;

pub fn ast_parser<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Vec<Spanned<Statement>>, RichErr<'src, 'tokens>>
{
    statement()
        .then_ignore(just(Token::SyntaxToken(";")).or_not())
        .repeated()
        .collect()
}

#[derive(Debug)]
pub enum ModuleError<'a> {
    Tokenizer(Rich<'a, char>),
    AstParser(Rich<'a, Token<'a>>),
}

impl ModuleError<'_> {
    pub fn span(&self) -> &SimpleSpan {
        match self {
            ModuleError::Tokenizer(err) => err.span(),
            ModuleError::AstParser(err) => err.span(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            ModuleError::Tokenizer(err) => err.to_string(),
            ModuleError::AstParser(err) => format!("{:?}", err),
        }
    }

    pub fn build_report(self, path: &str) -> ariadne::Report<(&str, std::ops::Range<usize>)> {
        ariadne::Report::build(ariadne::ReportKind::Error, path, self.span().start)
            .with_label(
                ariadne::Label::new((path, self.span().start..self.span().end))
                    .with_message(self.message())
                    .with_color(ariadne::Color::Red),
            )
            .finish()
    }
}

#[derive(Debug)]
pub struct TokenizationResult<'a> {
    pub tokens: Vec<(Token<'a>, SimpleSpan)>,
    pub errors: Vec<ModuleError<'a>>,
}

#[derive(Debug)]
pub struct Ast<'a> {
    pub statements: Vec<Spanned<Statement>>,
    pub errors: Vec<ModuleError<'a>>,
    pub tokens: Vec<(Token<'a>, SimpleSpan)>,
}

pub fn tokenize(source: &str) -> TokenizationResult<'_> {
    let (mut tokens, errors) = tokenizer().parse(source).into_output_errors();

    if let Some(tokens) = &mut tokens {
        insert_template_tokens(source, tokens);
    }

    TokenizationResult {
        tokens: tokens.unwrap_or_default(),
        errors: errors.into_iter().map(ModuleError::Tokenizer).collect(),
    }
}

pub fn create_ast<'a>(tokenization_result: &'a TokenizationResult) -> Ast<'a> {
    let tokens = tokenization_result.tokens.as_slice();

    let (ast, errors) = ast_parser()
        .parse(tokens.spanned((tokens.len()..tokens.len()).into()))
        .into_output_errors();

    Ast {
        tokens: tokens.to_vec(),
        statements: ast.unwrap_or_default(),
        errors: errors.into_iter().map(ModuleError::AstParser).collect(),
    }
}

impl<'a> Ast<'a> {
    pub fn top_level_declarations(&self) -> impl Iterator<Item = Spanned<Declaration>> + '_ {
        self.statements
            .iter()
            .filter_map(|Spanned { inner, span }| match inner {
                Statement::Declaration(declaration) => Some(declaration.clone().with_span(*span)),
                _ => None,
            })
    }
}