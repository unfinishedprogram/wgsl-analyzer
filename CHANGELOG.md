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
