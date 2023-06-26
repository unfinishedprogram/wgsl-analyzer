# WGSL-Analyzer

A VSCode extension providing validation and syntax highlighting of wgsl files

## About

This extension uses [Naga](https://github.com/gfx-rs/naga) under the hood for validation.

It is heavily inspired by [vscode-wgsl](https://github.com/PolyMeilex/vscode-wgsl) with the most significant difference being that the main dependency [Naga](https://github.com/gfx-rs/naga) is included directly via WebAssembly, rather than requiring the installation of a second package.

## Building

```sh
npm run build;
```