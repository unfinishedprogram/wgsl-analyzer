import {
    createConnection,
    ProposedFeatures,
    PublishDiagnosticsParams,
    TextDocumentSyncKind,
} from 'vscode-languageserver/node';
import { WGSLLanguageServer } from '../../dist/wgsl_language_server';

// Create LSP connection
const connection = createConnection(ProposedFeatures.all);

console.log("STARTING: wgsl-language-server");

const sendDiagnosticsCallback = (params: PublishDiagnosticsParams) =>
    connection.sendDiagnostics(params);

const wgsl_ls = new WGSLLanguageServer(sendDiagnosticsCallback);

connection.onNotification((...args) => wgsl_ls.onNotification(...args));
connection.onCompletion((...args) => JSON.parse(wgsl_ls.onCompletion(args[0])));
connection.onDocumentSymbol((arg) => JSON.parse(wgsl_ls.onDocumentSymbol(arg)));
connection.onTypeDefinition((arg) => JSON.parse(wgsl_ls.onTypeDefinition(arg)));

connection.onInitialize(() => {
    return {
        capabilities: {
            textDocumentSync: {
                openClose: true,
                save: true,
                change: TextDocumentSyncKind.Full,
            },
            completionProvider: {},
            documentSymbolProvider: true,
            typeDefinitionProvider: true,
            workspace: {
                workspaceFolders: { supported: true },
                fileOperations: {
                    didDelete: {
                        filters: [{ pattern: { glob: '**' } }],
                    },
                },
            },
        },
    };
});

connection.listen();
