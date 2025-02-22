use std::ops::Range;

use super::Token;

pub fn insert_template_tokens(source: &str, tokens: &mut Vec<(Token, Range<usize>)>) {
    let templates = find_templates(source);

    let is_start = |pos| templates.iter().any(|(start, _)| *start == pos);
    let is_end = |pos| templates.iter().any(|(_, end)| *end == pos);

    let mut res_tokens: Vec<(Token, Range<usize>)> = vec![];

    for (token, span) in tokens.iter() {
        if matches!(token, Token::Syntax("<<") | Token::Syntax(">>")) {
            let ident = match token {
                Token::Syntax("<<") => Token::TemplateArgsStart,
                Token::Syntax(">>") => Token::TemplateArgsEnd,
                _ => unreachable!(),
            };

            let (left, right) = (span.start, span.start + 1);
            let (left_tok, right_tok) = (Token::Syntax("<"), Token::Syntax(">"));
            let (left_span, right_span) = (left..(left + 1), (right..(right + 1)));

            match (
                is_end(left) || is_start(left),
                is_end(right) || is_start(right),
            ) {
                (true, true) => {
                    res_tokens.push((ident.clone(), left_span));
                    res_tokens.push((ident.clone(), right_span));
                    continue;
                }
                (true, false) => {
                    res_tokens.push((ident.clone(), left_span));
                    res_tokens.push((right_tok.clone(), right_span));
                    continue;
                }
                (false, true) => {
                    res_tokens.push((left_tok.clone(), left_span));
                    res_tokens.push((ident.clone(), right_span));
                    continue;
                }
                _ => {}
            }
        } else if is_start(span.start) {
            res_tokens.push((Token::TemplateArgsStart, span.clone()));
            continue;
        } else if is_end(span.start) {
            res_tokens.push((Token::TemplateArgsEnd, span.clone()));
            continue;
        }

        res_tokens.push((token.clone(), span.clone()));
    }

    *tokens = res_tokens;
}

// https://www.w3.org/TR/WGSL/#template-list-discovery
// This is required to disambiguate templates from comparison operators
// TODO: Modify this algorithm to work on tokens
fn find_templates(src: &str) -> Vec<(usize, usize)> {
    struct UnclosedCandidate {
        // Position offset in bytes
        position: usize,
        depth: u32,
    }

    let chars: Vec<char> = src.chars().collect();
    let mut discovered_template_lists = vec![];
    let mut pending: Vec<UnclosedCandidate> = vec![];
    let mut current_position: usize = 0;
    let mut nesting_depth: u32 = 0;

    let mut in_line_comment = false;
    let mut in_block_comment = false;

    let byte_position = |p| chars[0..p].iter().cloned().collect::<String>().len();

    while current_position < chars.len() {
        if in_line_comment {
            if chars[current_position] == '\n' {
                in_line_comment = false;
            }
            current_position += 1;
            continue;
        }

        if in_block_comment {
            if chars[current_position] == '*' && chars[current_position + 1] == '/' {
                in_block_comment = false;
                current_position += 2;
                continue;
            }
            current_position += 1;
            continue;
        }

        match chars[current_position] {
            '/' => {
                current_position += 1;
                if chars[current_position] == '/' {
                    in_line_comment = true;
                    current_position += 1;
                    continue;
                } else if chars[current_position] == '*' {
                    in_block_comment = true;
                    current_position += 1;
                    continue;
                }
            }
            '<' => {
                pending.push(UnclosedCandidate {
                    position: current_position,
                    depth: nesting_depth,
                });
                current_position += 1;
                if chars[current_position] == '<' || chars[current_position] == '=' {
                    pending.pop();
                    current_position += 1;
                    continue;
                }
            }
            '>' => match pending.last() {
                Some(unclosed) if unclosed.depth == nesting_depth => {
                    discovered_template_lists.push((
                        byte_position(unclosed.position),
                        byte_position(current_position),
                    ));
                    pending.pop();
                    current_position += 1;
                    continue;
                }
                _ => {
                    current_position += 1;
                    if chars[current_position] == '=' {
                        current_position += 1
                    }
                    continue;
                }
            },

            '(' | '[' => {
                nesting_depth += 1;
                current_position += 1;
                continue;
            }

            ')' | ']' => {
                loop {
                    pending.pop();
                    if pending.is_empty() || pending.last().unwrap().depth < nesting_depth {
                        break;
                    }
                }
                nesting_depth = nesting_depth.saturating_sub(1);
                current_position += 1;
                continue;
            }

            '!' => {
                current_position += 1;
                if chars[current_position] == '=' {
                    current_position += 1
                }
                continue;
            }

            '=' => {
                current_position += 1;
                if chars[current_position] != '=' {
                    nesting_depth = 0;
                    pending.clear();
                }
                current_position += 1;
                continue;
            }

            ';' | '{' | ':' => {
                nesting_depth = 0;
                pending.clear();
                current_position += 1;
            }

            '&' if chars[current_position + 1] == '&' => {
                loop {
                    pending.pop();
                    if pending.is_empty() || pending.last().unwrap().depth < nesting_depth {
                        break;
                    }
                }
                current_position += 2;
            }

            '|' if chars[current_position + 1] == '|' => {
                loop {
                    pending.pop();
                    if pending.is_empty() || pending.last().unwrap().depth < nesting_depth {
                        break;
                    }
                }
                current_position += 2;
            }
            _ => current_position += 1,
        };
    }

    discovered_template_lists
}
