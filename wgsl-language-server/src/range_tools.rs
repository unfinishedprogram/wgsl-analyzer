use lsp_types::Position;

pub fn string_range(string: &str, range: &lsp_types::Range) -> std::ops::Range<usize> {
    string_offset(string, &range.start)..string_offset(string, &range.end)
}

pub fn string_offset(string: &str, position: &Position) -> usize {
    let mut res = 0;

    for (index, line) in string.lines().enumerate() {
        if index == position.line as usize {
            return res + position.character as usize;
        }
        // newline chars should be included
        res += line.len() + 1;
    }

    0
}

pub fn position_at_char_offset(source: &str, char_offset: usize) -> Position {
    let mut offset = 0;
    for (index, line) in source.lines().enumerate() {
        if offset + line.len() >= char_offset {
            return Position {
                line: index as u32,
                character: (char_offset - offset) as u32,
            };
        }
        offset += line.len() + 1;
    }
    Position::default()
}

pub fn lsp_range_from_char_span(source: &str, span: &std::ops::Range<usize>) -> lsp_types::Range {
    let start = position_at_char_offset(source, span.start);
    let end = position_at_char_offset(source, span.end);
    lsp_types::Range { start, end }
}
