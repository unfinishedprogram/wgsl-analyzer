use lsp_types::{CompletionItem, CompletionItemKind, Position};

pub trait CompletionProvider {
    fn get_completions(&self, position: &Position) -> Vec<CompletionItem>;
}

pub fn new_completion_item(symbol: impl Into<String>, kind: CompletionItemKind) -> CompletionItem {
    CompletionItem {
        label: symbol.into(),
        kind: Some(kind),
        ..Default::default()
    }
}

pub fn detailed_completion_item(
    symbol: impl Into<String>,
    kind: CompletionItemKind,
    detail: impl Into<String>,
) -> CompletionItem {
    CompletionItem {
        label: symbol.into(),
        kind: Some(kind),
        detail: Some(detail.into()),
        ..Default::default()
    }
}
