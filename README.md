# MealVote

An app for planning meals for a family or individual

## Repository Layout

Non-code folders:

 - `build`: (Not checked into git) Where the web UI is compiled to
 - `res`: Asset files

Shared library crates:

 - `pages`: Crate for generating HTML pages for the ui
 - `schema`: Crate for handling requests and responses in the MuON format
 - `validation`: Crate for asserting validations for both the ui and server

Binary crates:

 - `server`: Crate that serves HTTP requests and stores the databases
 - `ui`: Crate that compiles to the WebAssembly user interface
 - `xtask`: Helper tools for building, linting, and testing

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

# Design

## Storage

The server stores two database files per instance:

 - `/meals.json` — The meals available to vote for
 - `/votes.json` — Who voted for which meals

## Pages

 - Meal list / login (default page), lists the meals available
 - Meal creation / editing page

## HTTP

 - PUT `/api/votes/meal`
   ```muon
   :::
   meal_id: int >0 <=1_200
   vote: bool
   :::
   ```
