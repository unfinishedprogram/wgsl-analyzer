use std::collections::{BTreeMap, HashMap};

use lsp_types::{
    CompletionItem, Diagnostic, DidChangeTextDocumentParams, Position, PublishDiagnosticsParams,
    TextDocumentItem, Url,
};
use naga::{
    front::wgsl::ParseError,
    valid::{Capabilities, ValidationError, ValidationFlags, Validator},
    Module, WithSpan,
};

use crate::{
    completion_provider::CompletionProvider, range_tools::string_range, util::Ether,
    wgsl_error::WgslError,
};

pub struct TrackedDocument {
    pub uri: Url,
    pub content: String,
    pub version: i32,
    pub module: Option<Module>,
    pub validation_error: Option<WithSpan<ValidationError>>,
    pub parse_error: Option<ParseError>,
}

impl TrackedDocument {
    pub fn compile_module(
        &mut self,
        validator: &mut Validator,
    ) -> (
        Option<&Module>,
        Option<Ether<ParseError, WithSpan<ValidationError>>>,
    ) {
        self.validation_error = None;
        self.parse_error = None;

        validator.reset();

        match naga::front::wgsl::parse_str(&self.content) {
            Ok(module) => {
                let module = self.module.insert(module);
                match validator.validate(module) {
                    Ok(_) => (Some(module), None),
                    Err(error) => (
                        Some(module),
                        Some(Ether::Right(self.validation_error.insert(error).to_owned())),
                    ),
                }
            }
            Err(error) => (
                None,
                Some(Ether::Left(self.parse_error.insert(error).to_owned())),
            ),
        }
    }

    pub fn get_diagnostics(&self) -> Vec<Diagnostic> {
        let parse_error = self
            .parse_error
            .as_ref()
            .map(|err| WgslError::from_parse_err(err, &self.content, &self.uri));

        let validation_error = self
            .validation_error
            .as_ref()
            .map(|err| WgslError::from_validation_err(err, &self.content, &self.uri));

        vec![validation_error, parse_error]
            .into_iter()
            .flatten()
            .map(|v| v.into())
            .collect()
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
            module: None,
            parse_error: None,
            validation_error: None,
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
                doc.compile_module(&mut self.validator);
            }
        }
    }

    pub fn remove(&mut self, uri: &Url) {
        self.documents.remove(uri);
    }

    pub fn get_diagnostics(&self) -> BTreeMap<Url, PublishDiagnosticsParams> {
        self.documents
            .iter()
            .map(|(k, v)| {
                (
                    k.to_owned(),
                    PublishDiagnosticsParams::new(k.to_owned(), v.get_diagnostics(), None),
                )
            })
            .collect::<BTreeMap<_, _>>()
    }

    pub fn get_completion(&self, url: &Url, position: &Position) -> Vec<CompletionItem> {
        if let Some(doc) = self.documents.get(url) {
            return doc.get_completion(position);
        }
        vec![]
    }
}
