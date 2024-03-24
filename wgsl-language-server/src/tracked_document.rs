use lsp_types::TextDocumentItem;
use wgsl_ast::module::Module;

use crate::diagnostic::wgsl_error_to_lsp_diagnostic;

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
            .map(|d| wgsl_error_to_lsp_diagnostic(self.lsp_doc.uri.clone(), &self.lsp_doc.text, d))
            .collect()
    }
}
