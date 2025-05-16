import {
  ExtensionContext,
  RelativePattern,
  Uri,
  WorkspaceFolder,
  WorkspaceFoldersChangeEvent,
  workspace,
} from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
} from "vscode-languageclient/browser";

const extensionName = "wgsl-analyzer";

const clients: Map<string, LanguageClient> = new Map();

export async function activate(context: ExtensionContext) {
  console.info("Starting WGSL Language Support...");

  context.asAbsolutePath("dist/server.js");

  const folders = workspace.workspaceFolders || [];
  for (const folder of folders) await startClient(folder, context);
  workspace.onDidChangeWorkspaceFolders(updateClients(context));
}

export async function deactivate(): Promise<void> {
  await Promise.all([...clients.values()].map((client) => client.stop()));
}

async function startClient(folder: WorkspaceFolder, context: ExtensionContext) {
  const server = Uri.joinPath(context.extensionUri, "dist/server.js");
  const worker = new Worker(server.toString(true));

  worker.postMessage({
    type: "webpack_public_path",
    extension_uri: context.extensionUri.toString(),
  });

  worker.postMessage({ type: "start" });

  console.error("Starting client");
  console.error("Server Module URI", server);

  const createChangeWatcher = workspace.createFileSystemWatcher(
    new RelativePattern(folder, "**/*.wgsl"),
    false,
    false,
    true
  );

  context.subscriptions.push(createChangeWatcher);

  const clientOpts: LanguageClientOptions = {
    documentSelector: [
      { language: "wgsl" },
    ],
    diagnosticCollectionName: extensionName,
    workspaceFolder: folder,
  };

  const client = new LanguageClient(
    extensionName,
    extensionName,
    clientOpts,
    worker
  );

  clients.set(folder.uri.toString(), client);
  await client.start();
}

async function stopClient(folder: string) {
  await clients.get(folder)?.stop();
  clients.delete(folder);
}

function updateClients(context: ExtensionContext) {
  return async function ({ added, removed }: WorkspaceFoldersChangeEvent) {
    console.log("Updating clients");

    // Clean up clients for removed folders.
    for (const folder of removed) await stopClient(folder.uri.toString());

    // Create clients for added folders.
    for (const folder of added) await startClient(folder, context);
  };
}
