use lsp_types::{DocumentSymbol, SymbolKind};
use naga::Handle;
use regex::Regex;

use crate::{document_tracker::TrackedDocument, range_tools::span_to_lsp_range};

pub enum SymbolItem {
    Constant(Handle<naga::Constant>),
    Function(Handle<naga::Function>),
    Struct(Handle<naga::Type>),
}

fn parse_function_signature(signature: &str) -> String {
    let signature = signature.replace('\n', " ");
    if let Some((_, [args, _, result])) = Regex::new(r"fn +.+? *\((.*?)\)\s*(->|)\s*(.*?) *?\{")
        .unwrap()
        .captures_iter(&signature)
        .map(|c| c.extract())
        .next()
    {
        let args: Vec<&str> = args
            .split_ascii_whitespace()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        let args = args.join(" ");

        let result = result.trim();
        if result.is_empty() {
            format!("fn ({})", args)
        } else {
            format!("fn ({}) -> {}", args, result)
        }
    } else {
        "<INVALID>".to_owned()
    }
}

impl<'a> SymbolItem {
    fn symbol_kind(&self) -> SymbolKind {
        match self {
            SymbolItem::Constant(_) => SymbolKind::Constant,
            SymbolItem::Function(_) => SymbolKind::Function,
            SymbolItem::Struct(_) => SymbolKind::Struct,
        }
    }

    fn parse_detail(&'a self, content: &'a str) -> String {
        match self {
            SymbolItem::Constant(_) => content.to_owned(),
            SymbolItem::Function(_) => parse_function_signature(content),
            SymbolItem::Struct(_) => content.to_owned(),
        }
    }
}

pub trait SymbolProvider {
    fn get_symbols(&self) -> Vec<DocumentSymbol>;
    fn document_symbol(&self, module: &naga::Module, item: SymbolItem) -> DocumentSymbol;
}

impl SymbolProvider for TrackedDocument {
    fn document_symbol(&self, module: &naga::Module, item: SymbolItem) -> DocumentSymbol {
        let name = match item {
            SymbolItem::Constant(handle) => &module.constants[handle].name,
            SymbolItem::Function(handle) => &module.functions[handle].name,
            SymbolItem::Struct(handle) => &module.types[handle].name,
        }
        .clone()
        .unwrap_or_default();

        let span = match item {
            SymbolItem::Constant(handle) => module.constants.get_span(handle),
            SymbolItem::Function(handle) => module.functions.get_span(handle),
            SymbolItem::Struct(handle) => module.types.get_span(handle),
        };

        let span_content = &self.content[span.to_range().unwrap_or_default()];

        let detail = Some(item.parse_detail(span_content).to_owned());

        #[allow(deprecated)]
        DocumentSymbol {
            name,
            kind: item.symbol_kind(),
            detail,
            range: span_to_lsp_range(span, &self.content),
            selection_range: span_to_lsp_range(span, &self.content),
            children: None,
            deprecated: None,
            tags: None,
        }
    }

    fn get_symbols(&self) -> Vec<DocumentSymbol> {
        let Some(Ok((module, _))) = &self.compilation_result else {
            return vec![];
        };

        let constants = module
            .constants
            .iter()
            .map(|(handle, _)| SymbolItem::Constant(handle));

        let functions = module
            .functions
            .iter()
            .map(|(handle, _)| SymbolItem::Function(handle));

        let structs = module.types.iter().flat_map(|(handle, ty)| {
            if ty.name.is_some() {
                Some(SymbolItem::Struct(handle))
            } else {
                None
            }
        });

        constants
            .chain(functions)
            .chain(structs)
            .map(|item| self.document_symbol(module, item))
            .collect()
    }
}
