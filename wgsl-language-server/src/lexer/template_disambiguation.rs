use std::{collections::HashSet, ops::Range};

use super::Token;

pub fn insert_template_tokens(source: &str, tokens: &mut Vec<(Token, Range<usize>)>) {
    let templates = find_templates(source);

    let starts: HashSet<usize> = templates.iter().map(|(start, _)| *start).collect();
    let ends: HashSet<usize> = templates.iter().map(|(_, end)| *end).collect();

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
                ends.contains(&left) || starts.contains(&left),
                ends.contains(&right) || starts.contains(&right),
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
        } else if starts.contains(&span.start) {
            res_tokens.push((Token::TemplateArgsStart, span.clone()));
            continue;
        } else if ends.contains(&span.start) {
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

    let chars: Vec<(usize, char)> = src.char_indices().collect();
    let mut discovered_template_lists = vec![];
    let mut pending: Vec<UnclosedCandidate> = vec![];
    let mut current_position: usize = 0;
    let mut nesting_depth: u32 = 0;

    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while current_position < chars.len() {
        let (byte_offset, current) = chars[current_position];
        let next = chars.get(current_position + 1).map(|it| it.1);

        if in_line_comment {
            if current == '\n' {
                in_line_comment = false;
            }
            current_position += 1;
            continue;
        }

        if in_block_comment {
            if current == '*' && next == Some('/') {
                in_block_comment = false;
                current_position += 2;
                continue;
            }
            current_position += 1;
            continue;
        }

        match current {
            '/' => {
                current_position += 1;
                if next == Some('/') || next == Some('*') {
                    in_line_comment = true;
                    current_position += 1;
                    continue;
                }
            }
            '<' => {
                pending.push(UnclosedCandidate {
                    position: byte_offset,
                    depth: nesting_depth,
                });
                current_position += 1;
                if matches!(next, Some('<' | '=')) {
                    pending.pop();
                    current_position += 1;
                    continue;
                }
            }
            '>' => match pending.last() {
                Some(unclosed) if unclosed.depth == nesting_depth => {
                    discovered_template_lists.push((unclosed.position, byte_offset));
                    pending.pop();
                    current_position += 1;
                    continue;
                }
                _ => {
                    current_position += 1;
                    if next == Some('=') {
                        current_position += 1;
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
                if next == Some('=') {
                    current_position += 1
                }
                continue;
            }

            '=' => {
                current_position += 2;
                if next != Some('=') {
                    nesting_depth = 0;
                    pending.clear();
                }
                continue;
            }

            ';' | '{' | ':' => {
                nesting_depth = 0;
                pending.clear();
                current_position += 1;
            }

            '&' if next == Some('&') => {
                loop {
                    pending.pop();
                    if pending.is_empty() || pending.last().unwrap().depth < nesting_depth {
                        break;
                    }
                }
                current_position += 2;
            }

            '|' if next == Some('|') => {
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
