use std::ops::Range;

use lsp_types::Position;

pub fn string_range(string: &str, range: lsp_types::Range) -> Range<usize> {
    position_in_string(string, range.start)..position_in_string(string, range.end)
}

pub fn position_in_string(string: &str, position: Position) -> usize {
    let mut res = 0;

    for (index, line) in string.lines().enumerate() {
        if index == position.line as usize {
            return res + position.character as usize;
        }
        res += line.len();
    }

    0
}
