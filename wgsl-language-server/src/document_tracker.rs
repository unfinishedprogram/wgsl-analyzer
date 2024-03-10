use std::collections::HashMap;

use lsp_types::{
    DidChangeTextDocumentParams, DocumentSymbol, Position, PublishDiagnosticsParams,
    TextDocumentItem, Url,
};
use wgsl_ast::module::Module;

use crate::{
    diagnostic::wgsl_error_to_lsp_diagnostic,
    range_tools::{new_location_link, string_offset, string_range},
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
        _ = self.compilation_result.insert(result);
    }

    pub fn get_lsp_diagnostics(&self) -> Vec<lsp_types::Diagnostic> {
        let diagnostics = match &self.compilation_result {
            Some(Ok(module)) => &module.diagnostics,
            Some(Err(diagnostics)) => diagnostics,
            None => return vec![],
        };

        diagnostics
            .iter()
            .map(|d| wgsl_error_to_lsp_diagnostic(self.uri.clone(), &self.content, d))
            .collect()
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
            module
                .module_scope
                .functions
                .get(&ident.inner)
                .map(|f| f.span)
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
