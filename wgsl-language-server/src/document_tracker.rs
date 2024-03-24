use std::collections::HashMap;

use lsp_types::{
    DidChangeTextDocumentParams, DocumentSymbol, Position, PublishDiagnosticsParams,
    TextDocumentItem, Url,
};

use crate::{symbol_provider::SymbolProvider, tracked_document::TrackedDocument};

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
        self.documents[url]
            .get_definition(position)
            .map(|link| link.into_lsp_location_link(&self.documents[url].lsp_doc.text, url))
    }

    pub fn get_symbols(&self) -> Vec<DocumentSymbol> {
        self.documents
            .values()
            .flat_map(|doc| doc.get_symbols())
            .collect()
    }
}
