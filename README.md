# MealVote

An app for planning meals for a family or individual

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
```

## Xtask

This project uses the cargo-xtask pattern.

```bash
# Build the release binary for the user interface
cargo xtask build-wasm
```
