use lsp_types::{Position, Range};

pub trait RangeTools {
    fn contains_line(&self, position: Position) -> bool;
    fn contains(&self, position: Position) -> bool;
}

impl RangeTools for Range {
    fn contains_line(&self, position: Position) -> bool {
        position.line >= self.start.line && position.line <= self.end.line
    }

    fn contains(&self, position: Position) -> bool {
        position >= self.start && position < self.end
    }
}
