use super::super::*;

#[cfg(test)]
mod int {
    use super::*;
    #[test]
    pub fn decimal_int_literals() {
        let literals = ["0", "0u", "0i", "1u", "123", "5346u"];

        for literal in literals.iter() {
            let mut lexer = Token::lexer(literal);
            assert_eq!(
                Some(Ok(Token::Integer(literal))),
                lexer.next(),
                "Lexer should parse integer decimal literal {:?}",
                literal
            );
        }
    }

    #[test]
    pub fn hex_int_literals() {
        let literals = [
            "0x0", "0x0u", "0x0i", "0x1u", "0x123", "0x5346u", "0X123u", "0x3f",
        ];

        for literal in literals.iter() {
            let mut lexer = Token::lexer(literal);
            assert_eq!(
                Some(Ok(Token::Integer(literal))),
                lexer.next(),
                "Lexer should parse integer hex literal {:?}",
                literal
            );
        }
    }
}

#[cfg(test)]
mod bool {
    use super::*;
    #[test]
    pub fn boolean_literals() {
        let bools = [("true", true), ("false", false)];

        for (source, value) in bools.iter() {
            let mut lexer = Token::lexer(source);
            assert_eq!(
                Some(Ok(Token::Boolean(*value))),
                lexer.next(),
                "Lexer should parse boolean literal {:?}",
                value
            );
        }
    }
}

#[cfg(test)]
mod float {
    use super::*;
    #[test]
    pub fn decimal_float_literals() {
        let literals = ["0.e+4f", "01.", ".01", "12.34", ".0f", "0h", "1e-3"];
        for literal in literals.iter() {
            let mut lexer = Token::lexer(literal);
            assert_eq!(
                Some(Ok(Token::Float(literal))),
                lexer.next(),
                "Lexer should parse float decimal literal {:?}",
                literal
            );
        }
    }

    #[test]
    pub fn hex_float_literals() {
        let literals = [
            "0xa.fp+2",
            "0x1P+4f",
            "0X.3",
            "0x3p+2h",
            "0X1.fp-4",
            "0x3.2p+2h",
        ];
        for literal in literals.iter() {
            let mut lexer = Token::lexer(literal);
            assert_eq!(
                Some(Ok(Token::Float(literal))),
                lexer.next(),
                "Lexer should parse float hex literal {:?}",
                literal
            );
        }
    }
}
