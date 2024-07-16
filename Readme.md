# Editor

A simple IDE written in Rust + Bevy.

### NOTE
This is in early development! It will contain bugs and is feature incomplete!

## Reporting Issues
Feel free to [open an issue at the repository](https://github.com/Cr4shd3v/crash_ide/issues/new).

## Features
- cool features will be listed here :)

## Platforms
### Linux
- best support
  - all features for a release must work before release
  - might get some exclusive features
- main development target

### Windows
- good support
  - all features for a release must work before release
- regular testing

### macOS
- limited support
- no automated testing
- may or may not work, but feel free to open issues regarding macOS

## Development
### Linux
Execute the setup file at `build/linux/setup.sh`.

After that, you can simply run `cargo run`.

#### Packaging
If you want to build a .deb file, you can use [cargo-deb](https://crates.io/crates/cargo-deb).
Configurations for cargo-deb already exist.

There is also support for packaging a .rpm file with [cargo-generate-rpm](https://crates.io/crates/cargo-generate-rpm).
Configurations for it are present as well.

### Windows cross-compilation
Execute the setup file at `build/windows/setup.sh`.

After that, you can simply run `cargo run`.

### macOs
Execute the setup file at `build/macos/setup.sh`.

After that, you can simply run `cargo run` and pray.
