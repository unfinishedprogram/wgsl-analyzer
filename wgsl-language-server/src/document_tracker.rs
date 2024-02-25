use std::collections::HashMap;

use lsp_types::{
    CompletionItem, DidChangeTextDocumentParams, DocumentSymbol, Location, Position,
    PublishDiagnosticsParams, TextDocumentItem, Url,
};
use wgsl_ast::{
    front::{span::SpanAble, token::Token},
    module::Module,
};

use crate::{
    diagnostic::wgsl_error_to_lsp_diagnostic,
    range_tools::{new_location, string_offset, string_range},
    // completion_provider::CompletionProvider,
    symbol_provider::SymbolProvider,
};

pub struct TrackedDocument {
    pub uri: Url,
    pub content: String,
    pub version: i32,
    pub compilation_result: Option<Result<Module, Vec<wgsl_ast::diagnostic::Diagnostic>>>,
    pub last_valid_module: Option<Module>,
}

impl TrackedDocument {
    pub fn compile_module(&mut self) {
        let result = wgsl_ast::module::Module::from_source(&self.content);
        self.compilation_result.insert(result);
    }

    pub fn get_lsp_diagnostics(&self) -> Vec<lsp_types::Diagnostic> {
        match &self.compilation_result {
            Some(Err(diagnostics)) => diagnostics
                .iter()
                .map(|d| wgsl_error_to_lsp_diagnostic(self.uri.clone(), &self.content, d))
                .collect(),
            _ => vec![],
        }
    }
}

#[derive(Default)]
pub struct DocumentTracker {
    documents: HashMap<Url, TrackedDocument>,
}

impl DocumentTracker {
    pub fn insert(&mut self, doc: TextDocumentItem) {
        let document = TrackedDocument {
            uri: doc.uri.to_owned(),
            content: doc.text.clone(),
            version: doc.version,
            compilation_result: None,
            last_valid_module: None,
        };

        self.documents.insert(doc.uri.clone(), document);
        self.documents.get_mut(&doc.uri).unwrap().compile_module();
    }

    pub fn update(&mut self, change: DidChangeTextDocumentParams) {
        if let Some(doc) = self.documents.get_mut(&change.text_document.uri) {
            for change in change.content_changes {
                if let Some(range) = change.range {
                    let range = string_range(&doc.content, &range);
                    doc.content.replace_range(range, &change.text);
                } else {
                    doc.content = change.text;
                }
            }
            doc.compile_module();
        }
    }

    pub fn remove(&mut self, uri: &Url) {
        self.documents.remove(uri);
    }

    pub fn get_diagnostics(&self) -> Vec<PublishDiagnosticsParams> {
        let mut diagnostics = vec![];

        for (url, document) in &self.documents {
            let lsp_diagnostics: Vec<_> = document.get_lsp_diagnostics().into_iter().collect();

            diagnostics.push(PublishDiagnosticsParams {
                uri: url.clone(),
                diagnostics: lsp_diagnostics,
                version: None,
            })
        }

        diagnostics
    }

    pub fn get_completion(&self, url: &Url, position: &Position) -> Vec<CompletionItem> {
        vec![]
    }

    pub fn get_type_definition(&self, url: &Url, position: &Position) -> Option<Location> {
        if let Some(Ok(module)) = &self.documents[url].compilation_result {
            module
                .token_at_position(string_offset(&module.source, position))
                .map(|t| match t {
                    Token::Ident(s) => Some(s),
                    _ => None,
                })
                .flatten()
                .map(|s| {
                    module
                        .type_store
                        .handle_of_ident(s.to_owned().with_span((0..0).into()))
                        .ok()
                })
                .flatten()
                .map(|handle| module.type_store.span_of(&handle))
                .flatten()
                .map(|span| new_location(span.into_range(), &module.source, url.clone()))
        } else {
            None
        }
    }

    pub fn get_symbols(&self) -> Vec<DocumentSymbol> {
        self.documents
            .values()
            .flat_map(|doc| doc.get_symbols())
            .collect()
    }
}
