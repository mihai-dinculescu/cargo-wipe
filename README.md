# Cargo Wipe

[![Crates][crates_badge]][crates]
[![CI][ci_badge]][ci]
[![codecov][codecov_badge]][codecov]
[![license][license_badge]][license]
[![Crates.io][crates_installs_badge]][crates]\
Cargo subcommand that recursively finds and optionally wipes all "target" or "node_modules" folders that are found in the current path.

## Usage

### Install

The [Rust toolchain][toolchain] is a prerequisite.

```bash
cargo install cargo-wipe
```

### Read the docs

```bash
cargo wipe --help
```

### Use

To find build folders for `<language>` that can potentially be deleted run

```bash
cargo wipe <language>
```

where `<language>` is `rust` or `node`. For example:

```bash
cargo wipe rust
```

This will run in dry-run mode and just print the list of directories to delete. To actually delete them run it again with the `-w` flag.

```bash
cargo wipe rust -w
```

Directories are found according to the following logic:

- `rust`: all directories called `target` containing a file called `.rustc_info.json`.
- `node`: all directories called `node_modules`.

You can use the `-i <path>` argument to ignore certain paths.

### Usage Example

![Usage Example Screenshot][usage_example]

## Contributions

Contributions are welcome and encouraged! See [/issues][issues] for ideas, or suggest your own!
If you're thinking to create a PR with large feature/change, please first discuss it in an issue.

### PR Checks

```bash
    cargo make ci-flow
```

### Releases

- Update version in `Cargo.toml`
- Update CHANGELOG.md
- Commit
- Add tag
  
  ```bash
  git tag -a vX.X.X
  ```

- Push

  ```bash
  git push --follow-tags
  ```

- Release\
  Create a [new release](https://github.com/mihai-dinculescu/cargo-wipe/releases). \
  `publish.yml` GitHub Action will pick it up and do the actual release to [crates.io][crates_io].

[crates_badge]: https://img.shields.io/crates/v/cargo-wipe.svg
[crates]: https://crates.io/crates/cargo-wipe
[ci_badge]: https://github.com/mihai-dinculescu/cargo-wipe/workflows/CI/badge.svg?branch=main
[ci]: https://github.com/mihai-dinculescu/cargo-wipe/actions
[codecov_badge]: https://codecov.io/gh/mihai-dinculescu/cargo-wipe/branch/main/graph/badge.svg
[codecov]: https://codecov.io/gh/mihai-dinculescu/cargo-wipe
[license_badge]: https://img.shields.io/crates/l/cargo-wipe.svg
[license]: https://github.com/mihai-dinculescu/cargo-wipe/blob/main/LICENSE
[crates_installs_badge]: https://img.shields.io/crates/d/cargo-wipe?label=cargo%20installs
[toolchain]: https://rustup.rs
[usage_example]: https://github.com/mihai-dinculescu/cargo-wipe/blob/main/assets/screenshot.PNG
[issues]: https://github.com/mihai-dinculescu/cargo-wipe/issues
[crates_io]: https://crates.io
