use super::{ident, Literal, RichErr, Token};

use chumsky::prelude::*;

pub fn literal<'src>() -> impl Parser<'src, &'src str, Token<'src>, RichErr<'src>> {
    let boolean_literal = choice((
        just("true").to(Literal::Boolean(true)),
        just("false").to(Literal::Boolean(false)),
    ));

    let int_literal = {
        let decimal = regex("0[iu]?").or(regex("[1-9][0-9]*[iu]?"));
        let hex = regex("0[xX][0-9a-fA-F]+[iu]?");

        choice((hex, decimal)).map(|s: &str| Literal::Int(s.to_string()))
    };

    let float_literal = {
        let decimal = choice((
            regex("0[fh]"),
            regex("[1-9][0-9]*[fh]"),
            regex("[0-9]*\\.[0-9]+([eE][+-]?[0-9]+)?[fh]?"),
            regex("[0-9]+\\.[0-9]*([eE][+-]?[0-9]+)?[fh]?"),
            regex("[0-9]+[eE][+-]?[0-9]+[fh]?"),
        ));

        let hex = choice((
            regex("0[xX][0-9a-fA-F]*\\.[0-9a-fA-F]+([pP][+-]?[0-9]+[fh]?)?"),
            regex("0[xX][0-9a-fA-F]+\\.[0-9a-fA-F]*([pP][+-]?[0-9]+[fh]?)?"),
            regex("0[xX][0-9a-fA-F]+[pP][+-]?[0-9]+[fh]?"),
        ));

        choice((decimal, hex)).map(|s: &str| Literal::Float(s.to_string()))
    };

    choice((float_literal, boolean_literal, int_literal)).map(Token::Literal)
}

pub fn syntax_token<'src>() -> impl Parser<'src, &'src str, Token<'src>, RichErr<'src>> {
    let l3 = choice((just("<<="), just(">>=")));

    let l2 = choice((
        just("=="),
        just("!="),
        just("<="),
        just(">="),
        just("&&"),
        just("||"),
        just("->"),
        just("=>"),
        just("++"),
        just("--"),
        just("+="),
        just("-="),
        just("*="),
        just("/="),
        just("%="),
        just("&="),
        just("|="),
        just("^="),
        just(">>"),
        just("<<"),
    ));

    let l1 = choice((
        just("("),
        just(")"),
        just("["),
        just("]"),
        just("{"),
        just("}"),
        just(";"),
        just("."),
        just(","),
        just(":"),
        just("&"),
        just("|"),
        just("^"),
        just("@"),
        just("="),
        just(">"),
        just("<"),
        just("%"),
        just("/"),
        just("+"),
        just("-"),
        just("*"),
        just("~"),
        just("!"),
    ));

    // Order is important here
    choice((l3, l2, l1)).map(Token::SyntaxToken)
}

pub fn trivia<'src>() -> impl Parser<'src, &'src str, Token<'src>, RichErr<'src>> {
    let line_comment = just("//")
        .then(none_of('\n').repeated())
        .padded()
        .to(Token::Trivia);

    let block_comment = {
        // TODO: Make this recursive
        let content = any().and_is(just("/*").or(just("*/")).not()).repeated();
        content.delimited_by(just("/*"), just("*/"))
    }
    .to(Token::Trivia);

    choice((line_comment, block_comment)).boxed()
}

pub fn tokenizer<'src>(
) -> impl Parser<'src, &'src str, Vec<(Token<'src>, SimpleSpan)>, RichErr<'src>> {
    let token = choice((trivia(), syntax_token(), literal(), ident()));
    token
        // Add spans to all tokens
        .map_with(|tok, e| (tok, e.span()))
        // Remove whitespace
        .padded()
        .padded_by(trivia().repeated())
        // Error recovery
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
        .boxed()
}
