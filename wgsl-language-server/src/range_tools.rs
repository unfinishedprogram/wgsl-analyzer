use lsp_types::{Location, Position};
use naga::{SourceLocation, Span};

pub trait RangeTools {
    fn contains_line(&self, position: &Position) -> bool;
}

impl RangeTools for lsp_types::Range {
    fn contains_line(&self, position: &Position) -> bool {
        position.line >= self.start.line && position.line <= self.end.line
    }
}

pub fn string_range(string: &str, range: lsp_types::Range) -> std::ops::Range<usize> {
    string_offset(string, &range.start)..string_offset(string, &range.end)
}

pub fn string_offset(string: &str, position: &Position) -> usize {
    let mut res = 0;

    for (index, line) in string.lines().enumerate() {
        if index == position.line as usize {
            return res + position.character as usize;
        }
        res += line.len() + 1;
    }

    0
}

pub fn position_at_byte_offset(source: &str, byte_offset: usize) -> Position {
    let mut abs_offset = 0;
    let mut character = 0;
    let mut line = 0;

    for char in source.chars() {
        abs_offset += char.len_utf8();

        if char == '\n' {
            line += 1;
            character = 0;
        } else {
            character += char.len_utf8();
        }

        if abs_offset >= byte_offset {
            break;
        }
    }

    Position::new(line, character as u32)
}

pub fn span_to_lsp_range(span: Span, source: &str) -> lsp_types::Range {
    let std::ops::Range { start, end } = span.to_range().unwrap_or_default();

    let start = position_at_byte_offset(source, start);
    let end = position_at_byte_offset(source, end);

    lsp_types::Range { start, end }
}

pub fn range_to_span(range: std::ops::Range<usize>) -> Span {
    Span::new(range.start as u32, range.end as u32)
}

pub fn source_location_to_range(
    location: Option<SourceLocation>,
    source: &str,
) -> Option<lsp_types::Range> {
    let location = location?;
    let start = position_at_byte_offset(source, location.offset as usize);
    let end = position_at_byte_offset(source, (location.offset + location.length) as usize);

    Some(lsp_types::Range { start, end })
}

pub fn new_location(range: std::ops::Range<usize>, source: &str, uri: lsp_types::Uri) -> Location {
    let std::ops::Range { start, end } = range;

    let start = position_at_byte_offset(source, start);
    let end = position_at_byte_offset(source, end);

    Location {
        uri,
        range: lsp_types::Range { start, end },
    }
}
