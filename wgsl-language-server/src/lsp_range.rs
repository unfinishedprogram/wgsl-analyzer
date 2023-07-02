use std::ops::Range;

use lsp_types::Position;
use naga::{SourceLocation, Span};

pub fn string_range(string: &str, range: lsp_types::Range) -> Range<usize> {
    string_offset(string, range.start)..string_offset(string, range.end)
}

pub fn string_offset(string: &str, position: Position) -> usize {
    let mut res = 0;

    for (index, line) in string.lines().enumerate() {
        if index == position.line as usize {
            return res + position.character as usize;
        }
        res += line.len();
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

    unreachable!("invalid char offset: {char_offset}, in source: {source}");
}

pub fn span_to_range(span: Span, source: &str) -> lsp_types::Range {
    let Range { start, end } = span.to_range().unwrap();

    let start = position_at_char_offset(source, start);
    let end = position_at_char_offset(source, end);

    lsp_types::Range { start, end }
}

pub fn source_location_to_range(
    location: Option<SourceLocation>,
    source: &str,
) -> Option<lsp_types::Range> {
    let Some(location) = location else {return None};

    let start = position_at_char_offset(source, location.offset as usize);
    let end = position_at_char_offset(source, (location.offset + location.length) as usize);

    Some(lsp_types::Range { start, end })
}
