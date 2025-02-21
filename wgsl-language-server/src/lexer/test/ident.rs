use super::super::*;

#[test]
pub fn ident_with_leading_underscore_is_valid() {
    let ident = "_validIdent";
    let mut lexer = Token::lexer(ident);
    assert_eq!(
        Some(Ok(Token::Ident(ident))),
        lexer.next(),
        "Leading underscore should be a valid token"
    );
}

#[test]
pub fn ident_with_double_leading_underscore_is_invalid() {
    let ident = "__invalidIdent";
    let mut lexer = Token::lexer(ident);
    assert_eq!(
        Some(Err(LexError::InvalidIdentifier(
            IdentError::DoubleLeadingUnderscore
        ))),
        lexer.next(),
        "Double leading underscore should not be valid"
    );
}

#[test]
pub fn ident_with_single_underscore_is_invalid() {
    let ident = "_";
    let mut lexer = Token::lexer(ident);
    assert_eq!(
        Some(Err(LexError::InvalidIdentifier(
            IdentError::SingleUnderscore
        ))),
        lexer.next(),
        "Single underscore should not be valid ident"
    );
}

#[test]
pub fn ident_with_non_ascii_is_valid() {
    // Examples taken from WGSL language specification
    let idents = [
        "Δέλτα",
        "réflexion",
        "Кызыл",
        "𐰓𐰏𐰇",
        "朝焼け",
        "سلام",
        "검정",
        "שָׁלוֹם",
        "गुलाबी",
        "փիրուզ",
    ];

    for ident in idents.iter() {
        let mut lexer = Token::lexer(ident);
        assert_eq!(
            Some(Ok(Token::Ident(ident))),
            lexer.next(),
            "Failed for {}",
            ident
        );
    }
}
