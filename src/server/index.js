import {
  BrowserMessageReader,
  BrowserMessageWriter,
  createConnection,
  TextDocumentSyncKind,
} from "vscode-languageserver/browser";

const loadWasmModule = (e) => {
  if (e.data.type === "webpack_public_path") {
    let extension_uri = e.data.extension_uri;
    __webpack_public_path__ += extension_uri.replace("file:///", "") + "/dist/";

    require("../../dist/pkg")
      .then(startServer)
      .catch((err) => {
        console.error("Failed ti initialize wasm module", err);
      })
      .finally(() => {
        removeEventListener("message", loadWasmModule);
      });
  }
};

console.log("Establishing LSP connections");
const messageReader = new BrowserMessageReader(self);
const messageWriter = new BrowserMessageWriter(self);

// Create LSP connection
const connection = createConnection(messageReader, messageWriter);
connection.onInitialize(() => {
  return {
    capabilities: {
      textDocumentSync: {
        openClose: true,
        save: true,
        change: TextDocumentSyncKind.Full,
      },
      completionProvider: {
        triggerCharacters: ["."],
      },
      documentFormattingProvider: true,
      documentSymbolProvider: true,
      workspace: {
        workspaceFolders: { supported: true },
        fileOperations: {
          didDelete: {
            filters: [{ pattern: { glob: "**" } }],
          },
        },
      },
    },
  };
});
connection.listen();

// Workaround for loading wasm modules from a worker context
addEventListener("message", loadWasmModule);

function startServer({ WGSLLanguageServer }) {
  console.log("WGSL wasm module loaded... Starting server");

  const wgsl_ls = new WGSLLanguageServer(connection.sendDiagnostics);

  connection.onNotification((...args) => wgsl_ls.onNotification(...args));

  connection.onCompletion((...args) =>
    JSON.parse(wgsl_ls.onCompletion(args[0]))
  );

  connection.onDocumentSymbol((arg) =>
    JSON.parse(wgsl_ls.onDocumentSymbol(arg))
  );

  connection.onDocumentFormatting((arg) => {
    let res = wgsl_ls.onDocumentFormatting(JSON.stringify(arg));
    if (res == undefined) {
      return res;
    } else {
      return JSON.parse(res);
    }
  });
}
