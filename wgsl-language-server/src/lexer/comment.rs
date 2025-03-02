use logos::Lexer;

use super::{LexError, Token};

pub fn lex_multiline_comment<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Result<&'a str, LexError> {
    let remainder_slice = lex.remainder();
    let mut comment_depth = 1;
    let mut pos = 0;

    loop {
        let next_comment_start = remainder_slice[pos..].find("/*");
        let next_comment_end = remainder_slice[pos..].find("*/");
        match (next_comment_start, next_comment_end) {
            (Some(start), Some(end)) => {
                if start < end {
                    comment_depth += 1;
                    pos += start + 2;
                } else {
                    comment_depth -= 1;
                    pos += end + 2;
                }
            }
            (Some(start), None) => {
                pos += start + 2;
            }
            (None, Some(end)) => {
                comment_depth -= 1;
                pos += end + 2;
            }
            (None, None) => {
                lex.bump(remainder_slice.len());
                return Err(LexError::UnterminatedComment);
            }
        }

        if comment_depth == 0 {
            lex.bump(pos);
            return Ok(lex.slice());
        }
    }
}
