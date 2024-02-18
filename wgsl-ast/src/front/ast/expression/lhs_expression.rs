use chumsky::prelude::*;

use super::{component_or_swizzle_specifier, expression, ComponentOrSwizzleSpecifier};
use crate::front::{
    ast::{ParserInput, RichErr},
    token::Token,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LHSUnaryOperator {
    Pointer,
    Deref,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LHSExpression {
    Unary(LHSUnaryOperator, Box<LHSExpression>),
    ComponentOrSwizzle(Box<LHSExpression>, Box<ComponentOrSwizzleSpecifier>),
    Ident(String),
    Paren(Box<LHSExpression>),
}

pub fn core_lhs_expression<'tokens, 'src: 'tokens>(
    lhs_expression: impl Parser<'tokens, ParserInput<'tokens, 'src>, LHSExpression, RichErr<'src, 'tokens>>
        + Clone,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, LHSExpression, RichErr<'src, 'tokens>> + Clone
{
    choice((
        select! {Token::Ident(ident) => LHSExpression::Ident(ident.to_owned())}.labelled("ident"),
        lhs_expression
            .clone()
            .delimited_by(just(Token::SyntaxToken("(")), just(Token::SyntaxToken(")")))
            .map(|expr| LHSExpression::Paren(Box::new(expr))),
    ))
    .memoized()
    .labelled("core LHS expression")
}

pub fn lhs_expression<'tokens, 'src: 'tokens>(
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, LHSExpression, RichErr<'src, 'tokens>> + Clone
{
    recursive(|this| {
        choice((
            select! {
                Token::SyntaxToken("&") => LHSUnaryOperator::Pointer,
                Token::SyntaxToken("*") => LHSUnaryOperator::Deref,
            }
            .then(this.clone())
            .map(|(op, expr)| LHSExpression::Unary(op, Box::new(expr))),
            core_lhs_expression(this.clone())
                .then(component_or_swizzle_specifier(expression()))
                .map(|(expr, specifier)| {
                    LHSExpression::ComponentOrSwizzle(Box::new(expr), Box::new(specifier))
                }),
            core_lhs_expression(this.clone()),
        ))
        .memoized()
    })
    .labelled("LHS expression")
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use chumsky::{input::Input, Parser};

    use crate::front::{
        ast::expression::{
            lhs_expression::LHSUnaryOperator, ComponentOrSwizzleSpecifier,
            ComponentOrSwizzleSpecifierInner,
        },
        token::{parse::tokenizer, template::insert_template_delimiters},
    };

    use super::{lhs_expression, LHSExpression};

    fn parse_from_source(source: &'static str) -> LHSExpression {
        let templated = insert_template_delimiters(source);
        let tokens = tokenizer().parse(&templated).unwrap();

        let ast = lhs_expression().parse(
            tokens
                .as_slice()
                .spanned((source.len()..source.len()).into()),
        );

        ast.unwrap()
    }

    #[test]
    fn ident() {
        assert_matches!(parse_from_source("foo"), LHSExpression::Ident(s) if s.as_str() == "foo");
    }

    #[test]
    fn member_ident() {
        let expr = parse_from_source("foo.bar");

        match expr {
            LHSExpression::ComponentOrSwizzle(lhs, rhs) => {
                match lhs.as_ref() {
                    LHSExpression::Ident(ident) => assert_eq!(ident.as_str(), "foo"),
                    _ => panic!("Expected Ident"),
                };

                match &rhs.as_ref().0 {
                    ComponentOrSwizzleSpecifierInner::MemberAccess(ident) => {
                        assert_eq!(ident.as_str(), "bar")
                    }
                    other => panic!("Expected Member Ident, found: {:?}", other),
                }
            }
            _ => panic!("Expected ComponentOrSwizzle"),
        }
    }

    #[test]
    fn unary_deref() {
        assert_eq!(
            parse_from_source("*foo"),
            LHSExpression::Unary(
                LHSUnaryOperator::Deref,
                Box::new(LHSExpression::Ident("foo".to_owned()))
            )
        );

        assert_eq!(
            parse_from_source("&foo"),
            LHSExpression::Unary(
                LHSUnaryOperator::Pointer,
                Box::new(LHSExpression::Ident("foo".to_owned()))
            )
        );
    }

    #[test]
    fn parenthesis() {
        assert_eq!(
            parse_from_source("(foo)"),
            LHSExpression::Paren(Box::new(LHSExpression::Ident("foo".to_owned())))
        );

        assert_eq!(
            parse_from_source("((foo))"),
            LHSExpression::Paren(Box::new(LHSExpression::Paren(Box::new(
                LHSExpression::Ident("foo".to_owned())
            ))))
        );

        assert_eq!(
            parse_from_source("((foo.bar))"),
            LHSExpression::Paren(Box::new(LHSExpression::Paren(Box::new(
                LHSExpression::ComponentOrSwizzle(
                    Box::new(LHSExpression::Ident("foo".to_owned())),
                    Box::new(ComponentOrSwizzleSpecifier(
                        ComponentOrSwizzleSpecifierInner::MemberAccess("bar".to_owned()),
                        None
                    ))
                )
            ))))
        );
    }
}
