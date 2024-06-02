# Editor

A simple IDE written in Rust + Bevy.

### NOTE
This is in early development! It will contain bugs and is feature incomplete!

## Reporting Issues
Feel free to [open an issue at the repository](https://github.com/Cr4shd3v/editor/issues/new).

## Features
- cool features will be listed here :)

## Platforms
### Linux
- best support
- main development target

### Windows
- secondary development target
- regular testing

### MacOS
- no support
- may or may not work, but feel free to open issues regarding MacOS

## Development
### Linux
You will need mold, which is a different linker than the default. It fastens development by a lot. 
Install it with the following command:
```
sudo apt-get install mold clang
```
If you want to build a .deb file, you can use [cargo-deb](https://crates.io/crates/cargo-deb).
Configurations for cargo-deb already exist.

You can execute the setup.sh file if you want all dependencies for development & packaging.

After these steps, you can simply run `cargo run`.
