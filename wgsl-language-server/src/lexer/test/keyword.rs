use super::super::*;

#[test]
pub fn keyword_takes_priority() {
    let keywords = [
        ("alias", Keyword::Alias),
        ("break", Keyword::Break),
        ("case", Keyword::Case),
        ("const", Keyword::Const),
        ("const_assert", Keyword::ConstAssert),
        ("continue", Keyword::Continue),
        ("continuing", Keyword::Continuing),
        ("default", Keyword::Default),
        ("diagnostic", Keyword::Diagnostic),
        ("discard", Keyword::Discard),
        ("else", Keyword::Else),
        ("enable", Keyword::Enable),
        ("fn", Keyword::Fn),
        ("for", Keyword::For),
        ("if", Keyword::If),
        ("let", Keyword::Let),
        ("loop", Keyword::Loop),
        ("override", Keyword::Override),
        ("requires", Keyword::Requires),
        ("return", Keyword::Return),
        ("struct", Keyword::Struct),
        ("switch", Keyword::Switch),
        ("var", Keyword::Var),
        ("while", Keyword::While),
    ];
    for (source, keyword) in keywords.iter() {
        let mut lexer = Token::lexer(source);
        assert_eq!(
            Some(Ok(Token::Keyword(*keyword))),
            lexer.next(),
            "Lexer should prioritize keyword {:?} over ident",
            keyword
        );
    }
}

#[test]
pub fn reserved_words_should_be_invalid() {
    let ident = "interface";
    let mut lexer = Token::lexer(ident);
    assert_eq!(
        Some(Err(LexError::InvalidIdentifier(
            IdentError::ReservedKeyword
        ))),
        lexer.next(),
        "Reserved words should not be valid"
    );
}
