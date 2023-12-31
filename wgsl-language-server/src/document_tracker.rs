use std::collections::HashMap;

use codespan_reporting::diagnostic::Diagnostic;
use lsp_types::{
    CompletionItem, DidChangeTextDocumentParams, DocumentSymbol, Position,
    PublishDiagnosticsParams, TextDocumentItem, Url,
};
use naga::{
    front::wgsl::ParseError,
    valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
    Module,
};

use crate::{
    completion_provider::CompletionProvider,
    range_tools::string_range,
    symbol_provider::SymbolProvider,
    wgsl_error::{codespan_to_lsp_diagnostic, parse_error_to_lsp_diagnostic},
};

pub struct TrackedDocument {
    pub uri: Url,
    pub content: String,
    pub version: i32,
    pub compilation_result: Option<CompilationResult>,
    pub last_valid_module: Option<Module>,
}

type CompilationResult = Result<(Module, Result<ModuleInfo, Diagnostic<()>>), ParseError>;

impl TrackedDocument {
    pub fn compile_module(&mut self, validator: &mut Validator) -> &CompilationResult {
        validator.reset();
        let result = match naga::front::wgsl::parse_str(&self.content) {
            Err(parse_error) => Err(parse_error),
            Ok(module) => {
                self.last_valid_module = Some(module.clone());
                let validation_result = validator.validate(&module, self.content.to_owned());
                Ok((module, validation_result))
            }
        };

        self.compilation_result.insert(result)
    }

    pub fn get_lsp_diagnostics(&self) -> Option<lsp_types::Diagnostic> {
        let Some(compilation_result) = &self.compilation_result else {
            return None;
        };

        match compilation_result {
            Err(parse_error) => Some(parse_error_to_lsp_diagnostic(
                parse_error,
                &self.content,
                &self.uri,
            )),
            Ok((_, Err(validation_error))) => Some(codespan_to_lsp_diagnostic(
                validation_error.clone(),
                None,
                &self.uri,
                &self.content,
            )),
            _ => None,
        }
    }
}

pub struct DocumentTracker {
    validator: Validator,
    documents: HashMap<Url, TrackedDocument>,
}

impl DocumentTracker {
    pub fn new() -> Self {
        Self {
            validator: naga::valid::Validator::new(ValidationFlags::all(), Capabilities::all()),
            documents: Default::default(),
        }
    }

    pub fn insert(&mut self, doc: TextDocumentItem) {
        let mut document = TrackedDocument {
            uri: doc.uri.to_owned(),
            content: doc.text.clone(),
            version: doc.version,
            compilation_result: None,
            last_valid_module: None,
        };

        document.compile_module(&mut self.validator);

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
            doc.compile_module(&mut self.validator);
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
        if let Some(doc) = self.documents.get(url) {
            return doc.get_completion(position);
        }
        vec![]
    }

    pub fn get_symbols(&self) -> Vec<DocumentSymbol> {
        self.documents
            .values()
            .flat_map(|doc| doc.get_symbols())
            .collect()
    }
}
