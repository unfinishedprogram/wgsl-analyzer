import {
    createConnection,
    ProposedFeatures,
    PublishDiagnosticsParams,
    TextDocumentSyncKind,
} from 'vscode-languageserver/node';
import { WGSLLanguageServer } from '../../dist/wgsl_language_server';

// Create LSP connection
const connection = createConnection(ProposedFeatures.all);
console.log("STARTING LSP");

const sendDiagnosticsCallback = (params: PublishDiagnosticsParams) =>
    connection.sendDiagnostics(params);
const wgsl_ls = new WGSLLanguageServer(sendDiagnosticsCallback);

connection.onNotification((...args) => {
    wgsl_ls.onNotification(...args);
});

connection.onInitialize(() => {
    return {
        capabilities: {
            textDocumentSync: {
                openClose: true,
                save: true,
                change: TextDocumentSyncKind.Full,
            },
            workspace: {
                workspaceFolders: { supported: true },
                fileOperations: {
                    didDelete: {
                        filters: [{ pattern: { /* matches: 'folder', */ glob: '**' } }],
                    },
                },
            },
        },
    };
});

connection.listen();
