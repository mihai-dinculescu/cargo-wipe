# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com](http://keepachangelog.com/).

## [Unreleased]

## [v0.2.0] - 2020-05-24
### Added
- Validation that checks if `target` folders are indeed Rust folders
- Gracefully handle access denied errors

### Fixed
- Fix crash caused by dirs without read permissions
- Fix message when empty directories are found

## [v0.1.3] - 2020-05-21
### Changed
- Internal improvements

## [v0.1.2] - 2020-05-16
### Fixed
- Fix wipe instructions

## [v0.1.1] - 2020-05-16
### Added
- Add extra warning for wiping `target` folders
- Add `node_modules` and `target` as valid subcommands

## [v0.1.0] - 2020-05-16
### Initial Release of cargo-wipe

[unreleased]: https://github.com/mihai-dinculescu/cargo-wipe
[v0.2.0]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.2.0
[v0.1.3]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.1.3
[v0.1.2]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.1.2
[v0.1.1]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.1.1
[v0.1.0]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.1.0
