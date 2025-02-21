use super::super::*;

#[test]
pub fn operators_take_priority() {
    let operators = [
        "<<=", ">>=", "==", "!=", "<=", ">=", "&&", "||", "->", "=>", "++", "--", "+=", "-=", "*=",
        "/=", "%=", "&=", "|=", "^=", ">>", "<<", "(", ")", "[", "]", "{", "}", ";", ".", ",", ":",
        "&", "|", "^", "@", "=", ">", "<", "%", "/", "+", "-", "*", "~", "!",
    ];

    for op in operators.iter() {
        let mut lexer = Token::lexer(op);
        assert_eq!(
            Some(Ok(Token::Syntax(op))),
            lexer.next(),
            "Lexer should prioritize operator {:?} over ident",
            op
        );
    }
}
