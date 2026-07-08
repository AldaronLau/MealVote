# MealVote

An app for planning meals for a family or individual

## Repository Layout

 - `build`: (Not checked into git) Where the web UI is compiled to
 - `res`: Asset files
 - `pages`: Crate for generating HTML pages for the ui
 - `schema`: Crate for handling requests and responses in the MuON format
 - `server`: Crate that serves requests and stores a database
 - `ui`: Crate that compiles to the WebAssembly user interface
 - `validation`: Crate for asserting validations for both the ui and server
 - `xtask`: Helper tools for development

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install Binaryen (for wasm-opt) and Wabt (for wasm-strip)
sudo dnf install binaryen wabt
# Install Cargo Binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
# Install Wasm-Bindgen
cargo binstall wasm-bindgen-cli -y
# Install simple http server for testing
cargo binstall simple-http-server
```

## Xtask

This project uses the cargo-xtask pattern.

```bash
# Build the release binary and HTML for the user interface
cargo xtask build-wasm
# Build just the HTML
cargo xtask build-html
```
