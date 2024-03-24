use lsp_types::{Position, TextDocumentItem};
use wgsl_ast::module::{declaration::function::Function, Module};

use crate::{
    diagnostic::wgsl_error_to_lsp_diagnostic,
    find_definition::{DefinitionLocation, DefinitionLocationProvider, FindDefinitionResult},
    range_tools::{string_offset, string_range},
};

pub struct TrackedDocument {
    pub lsp_doc: TextDocumentItem,
    pub compilation_result: Option<Result<Module, Vec<wgsl_ast::diagnostic::Diagnostic>>>,
    pub last_valid_module: Option<Module>,
}

impl From<TextDocumentItem> for TrackedDocument {
    fn from(document: TextDocumentItem) -> Self {
        TrackedDocument {
            lsp_doc: document,
            compilation_result: None,
            last_valid_module: None,
        }
    }
}

impl TrackedDocument {
    pub fn compile_module(&mut self) {
        let result = wgsl_ast::module::Module::from_source(&self.lsp_doc.text);
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
            .map(|d| wgsl_error_to_lsp_diagnostic(self.uri().clone(), &self.lsp_doc.text, d))
            .collect()
    }

    pub fn get_definition(&self, position: &Position) -> Option<FindDefinitionResult> {
        let Some(Ok(module)) = &self.compilation_result else {
            return None;
        };

        let ident = module.ident_at_position(&string_offset(&module.source, position))?;

        let type_def_location = {
            module
                .type_store
                .handle_of_ident(ident)
                .ok()
                .and_then(|type_handle| {
                    module
                        .type_store
                        .get_type(&type_handle)
                        .definition_location()
                })
        };

        let function_def_location = {
            let function = module.module_scope.functions.get(&ident.inner);
            if let Some(Function::UserDefined(f)) = function {
                Some(DefinitionLocation::new(
                    f.ident.span.into_range(),
                    f.span.into(),
                ))
            } else {
                None
            }
        };

        let def_location = type_def_location.or(function_def_location)?;

        Some(FindDefinitionResult::new(
            ident.span.into_range(),
            def_location,
        ))
    }

    pub fn apply_document_changes(
        &mut self,
        changes: &[lsp_types::TextDocumentContentChangeEvent],
    ) {
        changes
            .iter()
            .for_each(|change| self.apply_change_event(change));
        self.compile_module();
    }

    fn apply_change_event(&mut self, change: &lsp_types::TextDocumentContentChangeEvent) {
        if let Some(range) = change.range {
            let range = string_range(&self.lsp_doc.text, &range);
            self.lsp_doc.text.replace_range(range, &change.text);
        } else {
            self.lsp_doc.text = change.text.clone();
        }
    }
}

impl TrackedDocument {
    pub fn uri(&self) -> &lsp_types::Url {
        &self.lsp_doc.uri
    }
}
