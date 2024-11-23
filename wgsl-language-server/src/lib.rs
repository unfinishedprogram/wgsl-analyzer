mod block_ext;
mod completions;
mod document_tracker;
mod parser;
mod pretty_error;
mod range_tools;
mod symbol_provider;
mod wgsl_error;

use document_tracker::DocumentTracker;

use lsp_types::{
    CompletionItem, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, Position, PublishDiagnosticsParams, TextDocumentIdentifier,
    TextDocumentPositionParams,
};

use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_log(s: &str);
}

fn log(s: &str) {
    console_log(s)
}

#[wasm_bindgen]
pub struct WGSLLanguageServer {
    documents: DocumentTracker,
    send_diagnostics_callback: js_sys::Function,
}

#[wasm_bindgen]
impl WGSLLanguageServer {
    #[wasm_bindgen(constructor)]
    pub fn new(send_diagnostics_callback: &js_sys::Function) -> Self {
        console_error_panic_hook::set_once();
        log("WGSL Language Server Created");
        Self {
            documents: DocumentTracker::new(),
            send_diagnostics_callback: send_diagnostics_callback.clone(),
        }
    }

    #[wasm_bindgen(js_name = onCompletion)]
    pub fn on_completion(&mut self, params: JsValue) -> String {
        log("Request for completion");
        let TextDocumentPositionParams {
            text_document,
            position,
        } = from_value(params).unwrap();

        let res = self.get_auto_complete(text_document, position);
        serde_json::to_string(&res).unwrap()
    }

    #[wasm_bindgen(js_name = onDocumentSymbol)]
    pub fn on_document_symbol(&mut self, _params: JsValue) -> String {
        log("Request for document symbol");
        let res = self.documents.get_symbols();
        serde_json::to_string(&res).unwrap()
    }

    #[wasm_bindgen(js_name = onNotification)]
    pub fn on_notification(&mut self, method: &str, params: JsValue) {
        match method {
            "textDocument/didOpen" => {
                let DidOpenTextDocumentParams { text_document } = from_value(params).unwrap();
                self.documents.insert(text_document);
                self.update_diagnostics();
            }
            "textDocument/didClose" => {
                let params: DidCloseTextDocumentParams = from_value(params).unwrap();
                self.documents.remove(&params.text_document.uri);
                self.update_diagnostics();
            }
            "textDocument/didChange" => {
                let params: DidChangeTextDocumentParams = from_value(params).unwrap();
                self.documents.update(params);
                self.update_diagnostics();
            }
            "textDocument/didSave" => {}
            "initialized" => {}
            _ => log(&format!("on_notification {} {:?}", method, params)),
        }
    }
}

impl WGSLLanguageServer {
    fn update_diagnostics(&mut self) {
        let diagnostics = self.get_diagnostics();
        log(&format!("{diagnostics:?}"));
        self.send_diagnostics(diagnostics)
    }

    fn send_diagnostics(&self, diagnostics: Vec<PublishDiagnosticsParams>) {
        let this = &JsValue::null();
        for params in diagnostics {
            let params = &to_value(&params).unwrap();
            if let Err(e) = self.send_diagnostics_callback.call1(this, params) {
                log(&format!(
                    "send_diagnostics params:\n\t{:?}\n\tJS error: {:?}",
                    params, e
                ));
            }
        }
    }

    fn get_auto_complete(
        &self,
        text_document: TextDocumentIdentifier,
        position: Position,
    ) -> Vec<CompletionItem> {
        self.documents.get_completion(&text_document.uri, &position)
    }

    fn get_diagnostics(&self) -> Vec<PublishDiagnosticsParams> {
        self.documents.get_diagnostics()
    }
}
