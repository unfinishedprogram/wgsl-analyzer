# WGSL-Analyzer

> A VSCode extension providing validation and syntax highlighting of WGSL (Web GPU Shading Language) files

*If you encounter any issues, please report them on [github](https://github.com/unfinishedprogram/wgsl-analyzer/issues)*

## Features

- ✅ **Syntax highlighting of WGSL files**
- ✅ **Symbol auto-completion**
- ✅ **Syntax validation**
- ✅ **Correctness validation**
- ✅ **Document outline**
- ✅ **Context aware auto-completion**
  - ✅ Local Variables
  - ✅ Global Constants
  - ✅ Functions
  - 🚧 Keywords

## Planned Features

- 🚧 *Info on hover*
- 🚧 *Improved diagnostic messages*

## About

This extension is written in rust and uses Naga compiled to wasm to generate diagnostics. 
This means that the extension should work on any platform, and does not require any external binaries.

## Developing

### Install pre-requisites
```sh
# Install wasm-pack from source
cargo install wasm-pack

# Install NPM deps 
npm i
```