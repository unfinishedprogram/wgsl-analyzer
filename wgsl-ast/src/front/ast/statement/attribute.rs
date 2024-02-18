use chumsky::prelude::*;

use crate::front::ast::{
    expression::{expression, Expression},
    ParserInput, RichErr, Token,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub arguments: Vec<Expression>,
}

impl Attribute {
    fn parser<'tokens, 'src: 'tokens>(
    ) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Attribute, RichErr<'src, 'tokens>> + Clone
    {
        let name = select! {
            Token::Ident(ident) => ident.to_owned(),
            Token::Keyword(keyword) => keyword.into(),
        };

        let args = expression()
            .separated_by(just(Token::SyntaxToken(",")))
            .allow_trailing()
            .at_most(3)
            .collect()
            .delimited_by(just(Token::SyntaxToken("(")), just(Token::SyntaxToken(")")))
            .or_not()
            .map(|args| args.unwrap_or_default());

        just(Token::SyntaxToken("@"))
            .ignore_then(name)
            .then(args)
            .map(|(name, arguments)| Self { name, arguments })
    }

    pub fn list_parser<'tokens, 'src: 'tokens>(
    ) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Vec<Self>, RichErr<'src, 'tokens>> + Clone
    {
        Self::parser().repeated().collect().labelled("attributes")
    }
}
