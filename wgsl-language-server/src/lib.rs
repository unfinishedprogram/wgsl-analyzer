mod lsp_range;
mod wgsl_error;

use std::collections::{BTreeMap, HashMap};

use lsp_range::string_range;
use naga::{
    front::wgsl,
    valid::{Capabilities, ValidationFlags},
};

use lsp_types::{
    Diagnostic, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    Position, PublishDiagnosticsParams, Range, TextDocumentContentChangeEvent, TextDocumentItem,
    Url,
};

use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wgsl_error::WgslError;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_log(s: &str);
}

fn log(s: &str) {
    console_log(s)
}

type Diagnostics = BTreeMap<Url, PublishDiagnosticsParams>;

struct TrackedDocument {
    pub content: String,
    pub version: i32,
    pub uri: Url,
}

#[wasm_bindgen]
pub struct WGSLLanguageServer {
    documents: HashMap<Url, TrackedDocument>,
    send_diagnostics_callback: js_sys::Function,
}

#[wasm_bindgen]
impl WGSLLanguageServer {
    #[wasm_bindgen(constructor)]
    pub fn new(send_diagnostics_callback: &js_sys::Function) -> Self {
        console_error_panic_hook::set_once();
        log("WGSL Language Server Created");
        Self {
            documents: Default::default(),
            send_diagnostics_callback: send_diagnostics_callback.clone(),
        }
    }

    #[wasm_bindgen(js_class = PolarLanguageServer, js_name = onNotification)]
    pub fn on_notification(&mut self, method: &str, params: JsValue) {
        match method {
            "textDocument/didOpen" => {
                log("DID OPEN");
                let DidOpenTextDocumentParams { text_document } = from_value(params).unwrap();
                self.insert_document(text_document)
            }

            "textDocument/didClose" => {
                let params: DidCloseTextDocumentParams = from_value(params).unwrap();
                self.remove_document(&params.text_document.uri);
            }
            "textDocument/didChange" => {
                let params: DidChangeTextDocumentParams = from_value(params).unwrap();
                self.update_document(
                    &params.text_document.uri,
                    params.content_changes,
                    params.text_document.version,
                );
            }
            "textDocument/didSave" => {}
            "initialized" => {}
            _ => log(&format!("on_notification {} {:?}", method, params)),
        }
    }
}

impl WGSLLanguageServer {
    fn insert_document(&mut self, doc: TextDocumentItem) {
        self.documents.insert(
            doc.uri.clone(),
            TrackedDocument {
                content: doc.text,
                version: doc.version,
                uri: doc.uri,
            },
        );
        self.update_diagnostics();
    }

    fn remove_document(&mut self, uri: &Url) {
        self.documents.remove(uri);
        self.update_diagnostics();
    }

    fn update_document(
        &mut self,
        uri: &Url,
        changes: Vec<TextDocumentContentChangeEvent>,
        new_version: i32,
    ) {
        if let Some(doc) = self.documents.get_mut(uri) {
            doc.version = new_version;
            log("ChangeLen");
            log(&changes.len().to_string());
            for change in changes {
                if let Some(range) = change.range {
                    let range = string_range(&doc.content, range);
                    doc.content.replace_range(range, &change.text);
                } else {
                    doc.content = change.text;
                }
            }
            log(&doc.content);
        } else {
            log("Change on untracked doc")
        }
        self.update_diagnostics();
    }

    fn update_diagnostics(&self) {
        self.send_diagnostics(self.get_diagnostics())
    }

    fn send_diagnostics(&self, diagnostics: Diagnostics) {
        let this = &JsValue::null();
        for params in diagnostics.into_values() {
            let params = &to_value(&params).unwrap();
            if let Err(e) = self.send_diagnostics_callback.call1(this, params) {
                log(&format!(
                    "send_diagnostics params:\n\t{:?}\n\tJS error: {:?}",
                    params, e
                ));
            }
        }
    }

    fn naga_validate_wgsl(&self, src: &str) -> Result<(), WgslError> {
        let module = wgsl::parse_str(src).map_err(|err| WgslError::from_parse_err(err, src))?;
        let mut validator =
            naga::valid::Validator::new(ValidationFlags::all(), Capabilities::all());

        if let Err(error) = validator.validate(&module) {
            Err(WgslError::Validation {
                src: src.to_owned(),
                error,
            })
        } else {
            Ok(())
        }
    }

    fn wgsl_error_to_diagnostic(err: WgslError) -> Diagnostic {
        match err {
            WgslError::Validation { error, .. } => Diagnostic {
                message: error.to_string(),
                ..Default::default()
            },
            WgslError::Parser { error, line, pos } => Diagnostic {
                range: Range::new(
                    Position::new(line as u32 - 1, pos as u32 - 1),
                    Position::new(line as u32 - 1, pos as u32 - 1),
                ),
                message: error,
                ..Default::default()
            },
            WgslError::Io(_) => todo!(),
        }
    }

    fn get_diagnostics_for_doc(&self, document: &TrackedDocument) -> PublishDiagnosticsParams {
        let mut diagnostics: Vec<Diagnostic> = vec![];
        let diag = self.naga_validate_wgsl(&document.content);

        if let Err(err) = diag {
            diagnostics.push(Self::wgsl_error_to_diagnostic(err));
        }

        PublishDiagnosticsParams::new(document.uri.clone(), diagnostics, Some(document.version))
    }

    fn get_diagnostics(&self) -> Diagnostics {
        self.documents
            .iter()
            .map(|(uri, doc)| (uri.clone(), self.get_diagnostics_for_doc(doc)))
            .collect::<BTreeMap<_, _>>()
    }
}
