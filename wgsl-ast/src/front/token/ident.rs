use super::{Keyword, RichErr, Token};

use chumsky::prelude::*;

pub fn ident<'src>() -> impl Parser<'src, &'src str, Token<'src>, RichErr<'src>> {
    let underscore_start = just('_')
        .then(any().filter(|c| unicode_ident::is_xid_continue(*c) && *c != '_'))
        .then(
            any()
                .filter(|c| unicode_ident::is_xid_continue(*c))
                .repeated()
                .or_not(),
        )
        .to_slice();

    let normal_start = any()
        .filter(|c| unicode_ident::is_xid_start(*c))
        .then(
            any()
                .filter(|c| unicode_ident::is_xid_continue(*c))
                .repeated()
                .or_not(),
        )
        .to_slice();

    choice((underscore_start, normal_start))
        .map(Token::Ident)
        .map(|ident| match ident {
            Token::Ident("alias") => Token::Keyword(Keyword::Alias),
            Token::Ident("break") => Token::Keyword(Keyword::Break),
            Token::Ident("case") => Token::Keyword(Keyword::Case),
            Token::Ident("const") => Token::Keyword(Keyword::Const),
            Token::Ident("constAssert") => Token::Keyword(Keyword::ConstAssert),
            Token::Ident("continue") => Token::Keyword(Keyword::Continue),
            Token::Ident("continuing") => Token::Keyword(Keyword::Continuing),
            Token::Ident("default") => Token::Keyword(Keyword::Default),
            Token::Ident("diagnostic") => Token::Keyword(Keyword::Diagnostic),
            Token::Ident("discard") => Token::Keyword(Keyword::Discard),
            Token::Ident("else") => Token::Keyword(Keyword::Else),
            Token::Ident("enable") => Token::Keyword(Keyword::Enable),
            Token::Ident("fn") => Token::Keyword(Keyword::Fn),
            Token::Ident("for") => Token::Keyword(Keyword::For),
            Token::Ident("if") => Token::Keyword(Keyword::If),
            Token::Ident("let") => Token::Keyword(Keyword::Let),
            Token::Ident("loop") => Token::Keyword(Keyword::Loop),
            Token::Ident("override") => Token::Keyword(Keyword::Override),
            Token::Ident("requires") => Token::Keyword(Keyword::Requires),
            Token::Ident("return") => Token::Keyword(Keyword::Return),
            Token::Ident("struct") => Token::Keyword(Keyword::Struct),
            Token::Ident("switch") => Token::Keyword(Keyword::Switch),
            Token::Ident("var") => Token::Keyword(Keyword::Var),
            Token::Ident("while") => Token::Keyword(Keyword::While),
            v => v,
        })
        .boxed()
}
