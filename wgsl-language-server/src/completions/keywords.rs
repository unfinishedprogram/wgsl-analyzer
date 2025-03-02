use lsp_types::CompletionItemKind;

use super::{CompletionProvider, completion_provider::new_completion_item};

pub struct KeywordCompletions;

impl CompletionProvider for KeywordCompletions {
    fn get_completions(&self, _position: &lsp_types::Position) -> Vec<lsp_types::CompletionItem> {
        [
            "alias",
            "break",
            "case",
            "const",
            "const_assert",
            "continue",
            "continuing",
            "default",
            "diagnostic",
            "discard",
            "else",
            "enable",
            "false",
            "fn",
            "for",
            "if",
            "let",
            "loop",
            "override",
            "requires",
            "return",
            "struct",
            "switch",
            "true",
            "var",
            "while",
        ]
        .into_iter()
        .map(|it| new_completion_item(it, CompletionItemKind::KEYWORD))
        .collect()
    }
}
