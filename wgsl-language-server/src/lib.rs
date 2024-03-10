mod diagnostic;
mod document_tracker;
mod range_tools;
mod symbol_provider;

use document_tracker::DocumentTracker;

use lsp_types::{
    CompletionItem, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, LocationLink, Position, PublishDiagnosticsParams,
    TextDocumentIdentifier, TextDocumentPositionParams,
};

use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn setup_logging() {
    log::set_max_level(log::LevelFilter::Info);
    std::panic::set_hook(std::boxed::Box::new(console_error_panic_hook::hook));
    console_log::init().expect("could not initialize logger");
    log::info!("Logger Initialized");
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
        log::info!("WGSL Language Server Created");
        Self {
            documents: DocumentTracker::default(),
            send_diagnostics_callback: send_diagnostics_callback.clone(),
        }
    }

    #[wasm_bindgen(js_name = onCompletion)]
    pub fn on_completion(&mut self, params: JsValue) -> String {
        log::info!("Request for completion");

        let TextDocumentPositionParams {
            text_document,
            position,
        } = from_value(params).unwrap();

        let res = self.get_auto_complete(text_document, position);
        serde_json::to_string(&res).unwrap()
    }

    #[wasm_bindgen(js_name = onDocumentSymbol)]
    pub fn on_document_symbol(&mut self, _params: JsValue) -> String {
        log::info!("Request for document symbol");
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
            _ => log::info!("on_notification {} {:?}", method, params),
        }
    }

    #[wasm_bindgen(js_name = onDefinition)]
    pub fn on_definition(&mut self, params: JsValue) -> String {
        let TextDocumentPositionParams {
            text_document,
            position,
        } = from_value(params).unwrap();

        let res = self.get_definition(text_document, position);
        serde_json::to_string(&res).unwrap()
    }
}

impl WGSLLanguageServer {
    fn update_diagnostics(&mut self) {
        let diagnostics = self.get_diagnostics();
        log::info!("{diagnostics:?}");
        self.send_diagnostics(diagnostics)
    }

    fn send_diagnostics(&self, diagnostics: Vec<PublishDiagnosticsParams>) {
        let this = &JsValue::null();
        for params in diagnostics {
            let params = &to_value(&params).unwrap();
            if let Err(e) = self.send_diagnostics_callback.call1(this, params) {
                log::info!(
                    "send_diagnostics params:\n\t{:?}\n\tJS error: {:?}",
                    params,
                    e
                );
            }
        }
    }

    fn get_auto_complete(
        &self,
        _text_document: TextDocumentIdentifier,
        _position: Position,
    ) -> Vec<CompletionItem> {
        vec![]
    }

    fn get_diagnostics(&self) -> Vec<PublishDiagnosticsParams> {
        self.documents.get_diagnostics()
    }

    fn get_definition(
        &self,
        text_document: TextDocumentIdentifier,
        position: Position,
    ) -> Option<LocationLink> {
        self.documents.get_definition(&text_document.uri, &position)
    }
}
