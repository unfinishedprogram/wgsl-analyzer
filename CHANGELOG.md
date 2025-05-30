# Change Log

All notable changes to the "WGSL Language Server" extension will be documented in this file.

## [Unreleased]

- Initial release

## [0.2.0] 

### Added

- New Completion service
  - Globals
  - Functions
  - Function Local Variables
  - Types

### Changed

- More accurate validation error highlighting

### Fixed

- Incorrect usage of source field in LSP diagnostic

## [0.3.0]

### Added

- `DocumentSymbolProvider` for document outline view

### Changed

- Improved diagnostic messages

### Fixed

- Corrected details of completion options

## [0.3.1]

### Fixed

- [Incorrect diagnostic due to detection of entry-point outputs](https://github.com/unfinishedprogram/wgsl-analyzer/issues/1)

## [0.4.0]

### Changed

- Moved to mainline Naga
  - Can remain more up-to-date with Naga versions as they are released

### Fixed

- [#3](https://github.com/unfinishedprogram/wgsl-analyzer/issues/3) Is now fixed, since it was caused by a bug in a previous version of Naga

## [0.4.1]

### Changed

- Added keyword auto-complete
- Refactor of autocompletion system


## [0.4.2]

### Fixed

- Fixed incorrect error reporting locations on windows due to incorrect handling of CRLF line terminators see [issue #6](https://github.com/unfinishedprogram/wgsl-analyzer/issues/6)


## [0.4.3]

### Fixed

- Fixed panic when handling auto-completion after deleting lines

### Changed

- Updated to naga 23
- Improved error messages for nested expression errors


## [0.5.0]

### Added

- Type aware autocompletion of property access
- Autocompletion for builtin functions

### Fixed

- Updated Naga version to 24
- Adjustments to diagnostic messages
- Autocompletion is now more context aware



## [0.5.1]

### Changed

- Improved robustness and automation of releases


## [0.6.0]

### Added

- Code formatting for WGSL source files
  - Formatting style is inspired by rustfmt

### Changed

- Removed non-existent literals from language-configuration


## [0.6.1]

### Changed

- Updated rust edition to 2024
- Added package categories for new formatting feature

### Fixed

- Many formatting adjustments
  - A newline is added before attributes
  - Properties are separated with a newline for struct declarations
  - Whitespace between nested index accesses are removed
  - Whitespace surrounding index accesses are removed
  - Space between unary operations and targets has been removed
  - Fixed unwanted newlines in for loops
  - Removed newline within body of empty blocks
- Fixed potential panic when parsing nested block comments
- Massively improved formatting performance

## [0.7.0]

### Changed

- Now compatible with VS-Code for web!
