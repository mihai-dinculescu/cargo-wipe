# Cargo Wipe
[![Crates][crates_badge]][crates]\
Cargo subcommand that recursively finds and optionally wipes all "target" or "node_modules" folders that are found in the current path.

# Usage

## Install
The [Rust toolchain](https://rustup.rs) is a prerequisite.

```
cargo install cargo-wipe
```

## target
```
cargo wipe target
```
Add `-w` to wipe all folders found. USE WITH CAUTION!

## node_modules
```
cargo wipe node_modules
```
Add `-w` to wipe all folders found. USE WITH CAUTION!

## Usage Example
![alt text](https://github.com/mihai-dinculescu/cargo-wipe/blob/master/assets/screenshot.PNG "Usage Example")

# Contributions
Contributions are welcome and encouraged! See [TODO.md](TODO.md) for ideas, or suggest your own!

[crates_badge]: https://img.shields.io/crates/v/cargo-wipe.svg
[crates]: https://crates.io/crates/cargo-wipe