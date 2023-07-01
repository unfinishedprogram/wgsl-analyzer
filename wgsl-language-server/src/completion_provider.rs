use lsp_types::{CompletionItem, CompletionItemKind};
use naga::Module;

pub fn new_completion_item(symbol: String, kind: CompletionItemKind) -> CompletionItem {
    CompletionItem {
        label: symbol,
        kind: Some(kind),
        ..Default::default()
    }
}

pub fn get_completion(module: &Module) -> Vec<CompletionItem> {
    let constants = module.constants.iter().flat_map(|(_, v)| v.name.clone());
    let functions = module.functions.iter().flat_map(|(_, v)| v.name.clone());
    let types = module.types.iter().flat_map(|(_, v)| v.name.clone());

    use CompletionItemKind::*;

    let constants = constants.map(|v| new_completion_item(v, Constant));
    let functions = functions.map(|v| new_completion_item(v, Function));
    let types = types.map(|v| new_completion_item(v, Struct));

    constants.chain(functions).chain(types).collect()
}
