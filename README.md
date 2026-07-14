# MealVote

An app for planning meals for a family or individual

## Repository Layout

Non-code folders:

 - `build`: (Not checked into git) Where the web UI is compiled to
 - `res`: Asset files

Shared library crates:

 - `pages`: Crate for generating HTML pages for the ui
 - `schema`: Crate for handling requests and responses, and database MuON format
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
# Show help
cargo xtask
# Build the release binary and HTML for the user interface
cargo xtask build-wasm
# Build just the HTML
cargo xtask build-html
```

# Design

## Storage

The server stores two database files per instance:

 - `/users.muon` — Who exists in the system
   ```muon
   user: list record
     id: int >0 <=1_200
     name: text >0 <=64
     email: text >=6 <=254
     verified: bool
   ```
 - `/homes.muon` — Homes registered in the system
   ```muon
   home: list record
     id: int >0 <=1_024
     name: text >0 <=50
   ```

The server stores three database files per home (`/{home_id: int >0 <=1_024}/`):

 - `/meals.muon` — The meals available to vote for
   ```muon
   :::
   meal: list record
     id: int >0 <=1_200
     name: text >0 <=100
     desc: text <=32_000
   :::
   ```
 - `/votes.muon` — Who voted for which meals
   ```muon
   vote: list record
     id: int >0 <=1_000_000
     meal_id: int >0 <=1_200
     user_id: int >0 <=1_200
   ```
 - `/roles.muon` - Who has what role for the home
   ```muon
   vote: list record
     id: int >0 <=1_500_000
     home_id: int >0 <=1_024
     user_id: int >0 <=1_200
     role: list choice
       cook
       edit
       view
       vote
   ```

## Pages

 - Meal list / login (default page), lists the meals available
 - Meal creation / editing page

## HTTP API

All are at `/{meal_vote_api}`

### `/{home_id}/meals`
 
 - GET Reponse (role: **view**)
   ```muon
   :::
   meal: list record
     id: int >0 <=1_200
     name: text >0 <=100
   :::
   ```

### `/{home_id}/meals/{meal_id: int >0 <=1_200}`

 - GET Response (role: **view**)
   ```muon
   :::
   name: text >0 <=100
   desc: text <=32_000
   :::
   ```
 - POST Request (role: **edit**)
   ```muon
   :::
   name: text >0 <=100
   desc: text <=32_000
   ::: 
   ```
 - PATCH Request (role: **edit**)
   ```muon
   :::
   name: optional text >0 <=100
   desc: optional text <=32_000
   ::: 
   ```

### `/{home_id}/votes/{meal_id: int >0 <=1_200}`
 - PUT Request (role: **vote**)
   ```muon
   :::
   has_vote: bool
   :::
   ```

### `/{home_id}/roles/{user_id: int >0 <=1_200}`
 - GET Request (role: **edit**)
   ```muon
   :::
   cook: bool
   edit: bool
   view: bool
   vote: bool
   :::
   ```
 - PUT Request (role: **edit**)
   ```muon
   :::
   cook: bool
   edit: bool
   view: bool
   vote: bool
   :::
   ```
