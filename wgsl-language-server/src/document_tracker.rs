use std::collections::HashMap;

use lsp_types::{
    DidChangeTextDocumentParams, DocumentSymbol, Position, PublishDiagnosticsParams,
    TextDocumentItem, Url,
};
use wgsl_ast::module::declaration::function::Function;

use crate::{
    range_tools::{new_location_link, string_offset},
    symbol_provider::SymbolProvider,
    tracked_document::TrackedDocument,
};

#[derive(Default)]
pub struct DocumentTracker {
    documents: HashMap<Url, TrackedDocument>,
}

impl DocumentTracker {
    pub fn insert(&mut self, document: TextDocumentItem) {
        let mut document: TrackedDocument = document.into();
        document.compile_module();
        self.documents.insert(document.uri().clone(), document);
    }

    pub fn handle_change_text_document(&mut self, change: DidChangeTextDocumentParams) {
        if let Some(doc) = self.documents.get_mut(&change.text_document.uri) {
            doc.apply_document_changes(&change.content_changes);
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

    pub fn get_definition(
        &self,
        url: &Url,
        position: &Position,
    ) -> Option<lsp_types::LocationLink> {
        let Some(Ok(module)) = &self.documents[url].compilation_result else {
            return None;
        };

        let ident = module.ident_at_position(&string_offset(&module.source, position))?;
        // Search in types

        let type_def_span = {
            module
                .type_store
                .handle_of_ident(ident)
                .ok()
                .and_then(|type_handle| module.type_store.span_of(&type_handle))
        };

        let function_def_span = {
            let function = module.module_scope.functions.get(&ident.inner);
            if let Some(Function::UserDefined(f)) = function {
                Some(f.span)
            } else {
                None
            }
        };

        let dest_span = type_def_span.or(function_def_span)?;

        Some(new_location_link(
            ident.span.into_range(),
            dest_span.into_range(),
            &module.source,
            url.clone(),
        ))
    }

    pub fn get_symbols(&self) -> Vec<DocumentSymbol> {
        self.documents
            .values()
            .flat_map(|doc| doc.get_symbols())
            .collect()
    }
}
