use std::collections::HashMap;

use lsp_types::{
    CompletionItem, DidChangeTextDocumentParams, DocumentSymbol, Position,
    PublishDiagnosticsParams, TextDocumentItem, Url,
};
use wgsl_ast::module::Module;

use crate::{
    diagnostic::wgsl_error_to_lsp_diagnostic,
    range_tools::string_range,
    // completion_provider::CompletionProvider,
    symbol_provider::SymbolProvider,
};

pub struct TrackedDocument {
    pub uri: Url,
    pub content: String,
    pub version: i32,
    pub compilation_result: Option<Result<Module, wgsl_ast::diagnostic::Diagnostic>>,
    pub last_valid_module: Option<Module>,
}

impl TrackedDocument {
    pub fn compile_module(&mut self) -> &Result<Module, wgsl_ast::diagnostic::Diagnostic> {
        let result = wgsl_ast::module::Module::from_source(&self.content);
        self.compilation_result.insert(result)
    }

    pub fn get_lsp_diagnostics(&self) -> Option<lsp_types::Diagnostic> {
        match &self.compilation_result {
            Some(Err(diagnostic)) => Some(wgsl_error_to_lsp_diagnostic(
                self.uri.clone(),
                &self.content,
                diagnostic,
            )),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct DocumentTracker {
    documents: HashMap<Url, TrackedDocument>,
}

impl DocumentTracker {
    pub fn insert(&mut self, doc: TextDocumentItem) {
        let mut document = TrackedDocument {
            uri: doc.uri.to_owned(),
            content: doc.text.clone(),
            version: doc.version,
            compilation_result: None,
            last_valid_module: None,
        };

        document.compile_module();

        self.documents.insert(doc.uri, document);
    }

    pub fn update(&mut self, change: DidChangeTextDocumentParams) {
        if let Some(doc) = self.documents.get_mut(&change.text_document.uri) {
            for change in change.content_changes {
                if let Some(range) = change.range {
                    let range = string_range(&doc.content, range);
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

    pub fn get_symbols(&self) -> Vec<DocumentSymbol> {
        self.documents
            .values()
            .flat_map(|doc| doc.get_symbols())
            .collect()
    }
}
