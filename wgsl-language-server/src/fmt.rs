use lsp_types::FormattingOptions;

use crate::lexer::{lex, Token};

pub fn indent_string(options: &FormattingOptions) -> String {
    if options.insert_spaces {
        " ".repeat(options.tab_size as usize)
    } else {
        "\t".to_string()
    }
}

pub enum Delimiter {
    DoubleNewline,
    Newline,
    Space,
    None,
}

pub fn pretty_print_ast(code: &str, options: &FormattingOptions) -> Option<String> {
    let tokens = lex(code)?;

    let mut formatted = String::new();
    let mut indent_level: usize = 0;
    let indent_str = indent_string(options);

    for window in tokens.windows(2) {
        let (token, span) = &window[0];
        let (next_token, next_span) = &window[1];
        let src_content = &code[span.clone()].trim();

        // Whitespace will be anything skipped by the lexer
        let whitespace_span = span.end..next_span.start;
        let whitespace = &code[whitespace_span.clone()];

        let whitespace_lines = whitespace.chars().filter(|it| *it == '\n').count();
        let has_explicit_newline = if matches!(token, Token::Trivia(_)) {
            // The trialing newline of comments is part of the token
            // For the purposes of explicit newlines, we want to count it
            whitespace_lines > 0
        } else {
            whitespace_lines > 1
        };

        use Delimiter as D;
        use Token as T;

        let delimiter = match (token, next_token) {
            // Skip the delimiter for property access
            (_, T::Syntax(".")) | (T::Syntax("."), _) => D::None,

            (T::TemplateArgsStart, _) => D::None,
            (_, T::TemplateArgsEnd) => D::None,

            (T::Keyword(_) | T::Ident(_), T::TemplateArgsStart) => D::None,

            (T::Syntax("("), _) => D::None,
            (_, T::Syntax(")")) => D::None,

            (T::Syntax(";"), T::Syntax("}")) => {
                indent_level = indent_level.saturating_sub(1);
                D::Newline
            }
            (T::Syntax(";"), _) => D::Newline,
            (T::Syntax("{"), _) => {
                indent_level += 1;
                D::Newline
            }
            (_, T::Syntax("}")) => {
                indent_level = indent_level.saturating_sub(1);
                D::Newline
            }
            (T::Syntax("}"), _) => {
                if indent_level == 0 {
                    D::DoubleNewline
                } else {
                    D::Newline
                }
            }
            (T::Syntax("@"), _) => D::None,
            (_, T::Syntax(";")) => D::None,
            (_, T::Syntax(",")) => D::None,
            (T::Ident(_), T::Syntax(":")) => D::None,
            (T::Ident(_), T::Syntax("(")) => D::None,
            (T::Trivia(_), _) => D::Newline,
            (
                T::Keyword(_)
                | T::Boolean(_)
                | T::Ident(_)
                | T::Integer(_)
                | T::Float(_)
                | T::Syntax(_)
                | T::TemplateArgsEnd,
                _,
            ) => D::Space,
        };

        formatted.push_str(src_content);

        match delimiter {
            D::Newline => {
                formatted.push('\n');
                if has_explicit_newline {
                    formatted.push('\n');
                }
                formatted.push_str(&indent_str.repeat(indent_level));
            }
            D::DoubleNewline => formatted.push_str("\n\n"),
            D::Space => formatted.push(' '),
            D::None => {}
        }
    }

    let last_token = tokens.last()?;
    formatted.push_str(&code[last_token.1.clone()]);

    // Handle trailing whitespace
    let trailing_whitespace = &code[last_token.1.end..code.len()];
    let mut trailing_lines = trailing_whitespace.chars().filter(|it| *it == '\n').count();

    if matches!(options.trim_final_newlines, Some(true)) {
        trailing_lines = 0;
    }

    if matches!(options.insert_final_newline, Some(true)) {
        trailing_lines = trailing_lines.min(1);
    }

    for _ in 0..trailing_lines {
        formatted.push('\n');
    }

    Some(formatted)
}
