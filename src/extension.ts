import { ExtensionContext, RelativePattern, WorkspaceFolder, WorkspaceFoldersChangeEvent, languages, workspace } from "vscode";
import { join } from 'path';
import {
  LanguageClient,
  LanguageClientOptions,
  TransportKind
} from 'vscode-languageclient/node';

const extensionName = 'wgsl-analyzer';

const clients: Map<string, LanguageClient> = new Map();

function wgslFilesInWorkspacePattern(folder: WorkspaceFolder) {
  return new RelativePattern(folder, '**/*.wgsl');
}

export async function activate(context: ExtensionContext) {
  console.info("Starting WGSL Language Support...")

  const folders = workspace.workspaceFolders || [];
  for (const folder of folders) await startClient(folder, context);
  workspace.onDidChangeWorkspaceFolders(updateClients(context));
}

export async function deactivate(): Promise<void> {
  await Promise.all(
    [...clients.values()].map((client) => client.stop())
  );
}

async function startClient(folder: WorkspaceFolder, context: ExtensionContext) {
  const server = context.asAbsolutePath(join('dist', 'server.js'));
  console.error("Starting client");
  console.error("Server Module URI", server);

  const createChangeWatcher = workspace.createFileSystemWatcher(
    wgslFilesInWorkspacePattern(folder),
    false,
    false,
    true
  );

  context.subscriptions.push(createChangeWatcher);

  const debugOpts = {
    execArgv: ['--nolazy', `--inspect=${6011 + clients.size}`],
  };

  const serverOpts = {
    run: { module: server, transport: TransportKind.ipc },
    debug: { module: server, transport: TransportKind.ipc, options: debugOpts },
  };

  const clientOpts: LanguageClientOptions = {
    documentSelector: [
      { language: 'wgsl', pattern: `${folder.uri.fsPath}/**/*.wgsl` },
    ],
    diagnosticCollectionName: extensionName,
    workspaceFolder: folder,
  };

  const client = new LanguageClient(extensionName, serverOpts, clientOpts);
  clients.set(folder.uri.toString(), client);
  await client.start();
  console.log(client);
}

async function stopClient(folder: string) {
  const exists = clients.get(folder);
  if (exists) {
    const client = exists;
    await client.stop();
  }
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