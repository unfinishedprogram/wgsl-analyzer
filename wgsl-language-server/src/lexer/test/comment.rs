use super::super::*;

#[test]
pub fn comments_are_skipped() {
    let source = "/* This is a comment */ const";
    let mut lexer = Token::lexer(source);
    assert_eq!(
        Some(Ok(Token::Trivia("/* This is a comment */"))),
        lexer.next(),
        "Lexer should handle comments"
    );
    assert_eq!(
        Some(Ok(Token::Keyword(Keyword::Const))),
        lexer.next(),
        "Lexer should find keyword after comment"
    );
}

#[test]
pub fn multiline_comment_basic() {
    let source = "const /* This is a\nmultiline comment */ const";
    let mut lexer = Token::lexer(source);
    assert_eq!(
        Some(Ok(Token::Keyword(Keyword::Const))),
        lexer.next(),
        "Lexer should find keyword before multiline comment"
    );
    assert_eq!(
        Some(Ok(Token::Trivia("/* This is a\nmultiline comment */"))),
        lexer.next(),
        "Lexer should handle multiline comments"
    );
    assert_eq!(
        Some(Ok(Token::Keyword(Keyword::Const))),
        lexer.next(),
        "Lexer should find keyword after multiline comment"
    );
}

#[test]
pub fn multiline_comment_nested() {
    let source = "const /* outside first \n /* inside */ \n outside second */ const";
    let mut lexer = Token::lexer(source);
    assert_eq!(
        Some(Ok(Token::Keyword(Keyword::Const))),
        lexer.next(),
        "Lexer should find keyword before multiline comment"
    );
    assert_eq!(
        Some(Ok(Token::Trivia(
            "/* outside first \n /* inside */ \n outside second */"
        ))),
        lexer.next(),
        "Lexer should handle multiline comments"
    );
    assert_eq!(
        Some(Ok(Token::Keyword(Keyword::Const))),
        lexer.next(),
        "Lexer should find keyword after multiline comment"
    );
}

#[test]
pub fn multiline_comment_unterminated() {
    let source = "const /* This is an unterminated comment";
    let mut lexer = Token::lexer(source);
    assert_eq!(
        Some(Ok(Token::Keyword(Keyword::Const))),
        lexer.next(),
        "Lexer should find keyword before unterminated comment"
    );
    assert_eq!(
        Some(Err(LexError::UnterminatedComment)),
        lexer.next(),
        "Lexer should error on unterminated comment"
    );
}
