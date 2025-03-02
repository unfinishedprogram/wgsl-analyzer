use lsp_types::FormattingOptions;

use crate::lexer::{Keyword, Token, lex};

pub enum Delimiter {
    DoubleNewline,
    Newline,
    Space,
    None,
}

pub fn pretty_print_ast(code: &str, options: &FormattingOptions) -> Option<String> {
    let tokens = lex(code)?;
    let mut ctx = ASTContext::new(options);

    let mut formatted = String::new();

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

        ctx.handle_brackets((token, next_token));

        let delimiter = match (token, next_token) {
            // Skip the delimiter for property access
            (_, T::Syntax(".")) | (T::Syntax("."), _) => D::None,

            (T::TemplateArgsStart, _) => D::None,
            (_, T::TemplateArgsEnd) => D::None,

            (T::Keyword(_) | T::Ident(_), T::TemplateArgsStart) => D::None,

            (T::Syntax("("), _) => D::None,
            (_, T::Syntax(")")) => D::None,

            (T::Syntax("["), _) => D::None,
            (_, T::Syntax("]")) => D::None,

            (T::Ident(_) | T::Syntax("]"), T::Syntax("[")) => D::None,
            (T::Syntax("{"), T::Syntax("}")) => D::Space,

            (_, T::Syntax("}")) => {
                ctx.dedent();
                D::Newline
            }
            (T::Syntax("{"), _) => {
                ctx.indent();
                D::Newline
            }
            (T::Syntax(";"), _) => {
                if ctx.paren_level > 0 {
                    D::Space
                } else {
                    D::Newline
                }
            }
            (T::Syntax("}"), _) => {
                if ctx.indent_level == 0 {
                    D::DoubleNewline
                } else {
                    D::Newline
                }
            }

            // Unary Operators
            (T::Syntax("!" | "~"), _) => D::None,

            // Ptr operations
            (T::Syntax("*" | "&"), _) if matches!(ctx.prev_token, T::Syntax("=" | "(")) => D::None,

            // Newlines for struct declaration properties
            (T::Syntax(","), _) => {
                if ctx.brace_level == 1
                    && ctx.template_level == 0
                    && ctx.paren_level == 0
                    && ctx.bracket_level == 0
                {
                    D::Newline
                } else {
                    D::Space
                }
            }

            (_, T::Keyword(Keyword::Fn | Keyword::Var | Keyword::Const))
                if (ctx.template_level == 0 && ctx.paren_level == 0 && ctx.bracket_level == 0) =>
            {
                D::Newline
            }

            (T::Syntax("@"), _) => D::None,
            (_, T::Syntax(";")) => D::None,
            (_, T::Syntax(",")) => D::None,
            (T::Ident(_) | T::Syntax("]"), T::Syntax("++" | "--")) => D::None,
            (T::Ident(_), T::Syntax(":")) => D::None,
            (T::Ident(_) | T::TemplateArgsEnd, T::Syntax("(")) => D::None,
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
                formatted.push_str(&ctx.indentation());
            }
            D::DoubleNewline => formatted.push_str("\n\n"),
            D::Space => formatted.push(' '),
            D::None => {}
        }

        ctx.prev_token = token;
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

struct ASTContext<'a> {
    indent_level: usize,
    indent_str: String,
    paren_level: usize,
    bracket_level: usize,
    brace_level: usize,
    template_level: usize,
    // Used for differentiating between binary and unary operators
    prev_token: &'a Token<'a>,
}

impl ASTContext<'_> {
    fn new(options: &FormattingOptions) -> Self {
        let indent_str = if options.insert_spaces {
            " ".repeat(options.tab_size as usize)
        } else {
            "\t".to_string()
        };
        Self {
            indent_level: 0,
            indent_str,
            paren_level: 0,
            bracket_level: 0,
            brace_level: 0,
            template_level: 0,
            prev_token: &Token::Syntax(""),
        }
    }

    fn handle_brackets(&mut self, tokens: (&Token<'_>, &Token<'_>)) {
        match tokens {
            (Token::Syntax("{"), _) => {
                self.brace_level = self.brace_level.saturating_add(1);
            }
            (Token::Syntax("}"), _) => {
                self.brace_level = self.brace_level.saturating_sub(1);
            }

            (Token::Syntax("("), _) => {
                self.paren_level = self.paren_level.saturating_add(1);
            }
            (Token::Syntax(")"), _) => {
                self.paren_level = self.paren_level.saturating_sub(1);
            }

            (Token::Syntax("["), _) => {
                self.bracket_level = self.bracket_level.saturating_add(1);
            }
            (Token::Syntax("]"), _) => {
                self.bracket_level = self.bracket_level.saturating_sub(1);
            }

            (Token::TemplateArgsStart, _) => {
                self.template_level = self.template_level.saturating_add(1);
            }
            (Token::TemplateArgsEnd, _) => {
                self.template_level = self.template_level.saturating_sub(1);
            }

            _ => {}
        }
    }

    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn dedent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }

    fn indentation(&self) -> String {
        self.indent_str.repeat(self.indent_level)
    }
}
