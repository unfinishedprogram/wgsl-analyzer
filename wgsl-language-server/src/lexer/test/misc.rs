use super::super::*;
#[test]
pub fn whitespace_is_skipped() {
    let source = "  \t\n  const";
    let mut lexer = Token::lexer(source);
    assert_eq!(
        Some(Ok(Token::Keyword(Keyword::Const))),
        lexer.next(),
        "Lexer should skip whitespace"
    );
}
