# Change Log

All notable changes to this project will be documented in this
file. This change log follows the conventions of
[keepachangelog.com](http://keepachangelog.com/).

## [Unreleased]

### Changed

- All dependencies have been updated to their latest version.

## [v0.3.2] - 2021-10-31

### Changed

- Upgraded to Rust 2021

### Fixed

- The argument parsing has been improved so that it is less confusing

### Removed

- The "target" and "node_modules" aliases have been deprecated and removed from the documentation. They will continue to work for a few more releases, but everyone is encouraged to switch to the preferred "rust" and "node" namings.

### Added

- Specific paths can now be ignored by using the `i` argument

## [v0.3.1] - 2021-01-11

### Added

- Specific paths can now be ignored by using the `i` argument

## [v0.3.0] - 2020-10-17

### Added

- Before and after path disk size is now shown

### Changed

- Total space (that can be) made available is now shown in an appropriate unit (bytes, KiB, MiB, GiB, etc.)
- The `-w` flag can now be used both before and after the folder name argument
- The folder name is now an argument instead of a subcommand

### Fixed

- The `-w` flag now shows up in the root help section

## [v0.2.0] - 2020-05-24

### Added

- Validation that checks if `target` folders are indeed Rust folders

### Removed

- Extra warning for wiping `target` folders

### Fixed

- Access denied errors are now handled gracefully
- Fixed a crash caused by dirs without read permissions
- Fixed the message displayed when empty directories are found

## [v0.1.3] - 2020-05-21

### Changed

- Internal improvements

## [v0.1.2] - 2020-05-16

### Fixed

- Fix wipe instructions

## [v0.1.1] - 2020-05-16

### Added

- Extra warning for wiping `target` folders
- `node_modules` and `target` subcommands

## [v0.1.0] - 2020-05-16

### Initial Release of cargo-wipe

[unreleased]: https://github.com/mihai-dinculescu/cargo-wipe
[v0.3.2]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.3.2
[v0.3.1]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.3.1
[v0.3.0]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.3.0
[v0.2.0]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.2.0
[v0.1.3]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.1.3
[v0.1.2]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.1.2
[v0.1.1]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.1.1
[v0.1.0]: https://github.com/mihai-dinculescu/cargo-wipe/tree/v0.1.0
