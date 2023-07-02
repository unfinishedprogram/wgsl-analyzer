use std::{env::var, fmt::Debug};

use lsp_types::{CompletionItem, CompletionItemKind, Position, Range};
use naga::{Arena, Function, Handle, Module, Type, UniqueArena};

use crate::{
    lsp_range::{source_location_to_range, span_to_range},
    parser::matching_bracket_index,
    range_tools::RangeTools,
};

pub fn new_completion_item(symbol: String, kind: CompletionItemKind) -> CompletionItem {
    CompletionItem {
        label: symbol,
        kind: Some(kind),
        ..Default::default()
    }
}

pub fn detailed_completion_item(
    symbol: String,
    kind: CompletionItemKind,
    detail: &str,
) -> CompletionItem {
    CompletionItem {
        label: symbol,
        kind: Some(kind),
        detail: Some(detail.to_owned()),
        ..Default::default()
    }
}

pub fn get_function_scope_range(
    arena: &Arena<naga::Function>,
    function: naga::Handle<naga::Function>,
    src: &str,
) -> Range {
    let mut location = arena.get_span(function).location(src);

    if let Some(close) = matching_bracket_index(src, location.offset as usize) {
        location.length = close as u32 - location.offset;
    }

    source_location_to_range(Some(location), src).unwrap()
}

pub fn get_function_completion(
    function: &Function,
    content: &str,
    position: &Position,
) -> Vec<CompletionItem> {
    let mut res = vec![];

    for (handle, name) in function.named_expressions.iter() {
        let range = span_to_range(function.expressions.get_span(*handle), content);
        res.push(detailed_completion_item(
            name.to_owned(),
            CompletionItemKind::Variable,
            &format!("{:?}", range),
        ))
    }

    for (handle, variable) in function.local_variables.iter() {
        let range = span_to_range(function.local_variables.get_span(handle), content);

        if range.start < *position {
            if let Some(name) = &variable.name {
                res.push(new_completion_item(
                    name.to_owned(),
                    CompletionItemKind::Variable,
                ))
            }
        } else if let Some(name) = &variable.name {
            res.push(new_completion_item(
                name.to_owned(),
                CompletionItemKind::Variable,
            ))
        }
    }

    res
}

pub fn get_type_completion(
    types: &UniqueArena<Type>,
    content: &str,
    position: &Position,
) -> Vec<CompletionItem> {
    let mut res = vec![];

    for (_, ty) in types.iter() {
        if let Some(name) = &ty.name {
            res.push(detailed_completion_item(
                name.to_owned(),
                CompletionItemKind::Class,
                &format!("{:?}", ty.inner),
            ))
        }
    }
    res
}

pub fn get_completion(module: &Module, content: &str, position: Position) -> Vec<CompletionItem> {
    let mut res = vec![];

    for (handle, func) in module.functions.iter() {
        if let Some(name) = func.name.clone() {
            let r = get_function_scope_range(&module.functions, handle, content);

            // If within function scope, suggest local variables
            if r.contains_line(position) {
                res.extend(get_function_completion(func, content, &position))
            }

            res.push(new_completion_item(name, CompletionItemKind::Function))
        }
    }

    // let constants = module.constants.iter().flat_map(|(_, v)| v.name.clone());
    // use CompletionItemKind::*;

    // let constants = constants.map(|v| new_completion_item(v, Constant));
    // let functions = functions.map(|v| new_completion_item(v, Function));
    res.extend(get_type_completion(&module.types, content, &position));
    res

    // constants.chain(functions).chain(types).collect()
}
