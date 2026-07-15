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
   :::
   user: list record
     id: int >0 <=1_200
     name: text >0 <=64
     email: text >=6 <=254
     verified: bool
     # Generated code for email authentication
     code: optional text >=50 <=50
     # Salted password/passcode (unsalted pass acessed by email + code)
     pass: optional text >=50 <=50
     # Generated salt (regenerated for each new pass)
     salt: optional text >=50 <=50
     # Code or password/passcode expiry (only one allowed)
     expiry: datetime
   :::
   ```
 - `/homes.muon` — Homes registered in the system
   ```muon
   :::
   home: list record
     id: int >0 <=1_024
     name: text >0 <=50
     user_id: int >0 <=1_200
   :::
   ```

The server stores four database files per home (`/{home_id: int >0 <=1_024}/`):

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
   :::
   vote: list record
     id: int >0 <=1_000_000
     meal_id: int >0 <=1_200
     user_id: int >0 <=1_200
   :::
   ```
 - `/roles.muon` - Which roles exist for the home
   ```muon
   :::
   role: list record
     id: int >0 <=16
     name: text >=3 <=32
     actions: list choice
       cook
       edit
       view
       vote
   :::
   ```
 - `/perms.muon` - Role permission settings for each user
   ```muon
   :::
   perm: list record
     id: int >0 <=1_200
     role_id: int >0 <=16
     user_id: list int >0 <=1_200
   :::
   ```

## Pages

 - Meal list / login (default page), lists the meals available
 - Meal creation / editing page
 - Invite, permissions, and role editing page

## HTTP API

All are at `/{meal_vote_api}`

### `/`

 - POST Request (2-step authentication; email, email + pass)
   ```muon
   :::
   email: text >=6 <=254
   pass: optional text >=50 <=50
   :::
   ```

### `/homes/{home_id: int >0 <=1_024}`

 - GET Reponse (action: **view**)
   ```muon
   :::
   name: text >0 <=50
   actions: list choice
     cook
     edit
     view
     vote
   :::
   ```

### `/homes/{home_id}/meals`
 
 - GET Reponse (action: **view**)
   ```muon
   :::
   meal: list record
     id: int >0 <=1_200
     name: text >0 <=100
   :::
   ```
 - POST Request (action: **edit**)
   ```muon
   :::
   name: text >0 <=100
   desc: text <=32_000
   ::: 
   ```

### `/homes/{home_id}/meals/{meal_id: int >0 <=1_200}`

 - GET Response (action: **view**)
   ```muon
   :::
   desc: text <=32_000
   :::
   ```
 - PATCH Request (action: **edit**)
   ```muon
   :::
   name: optional text >0 <=100
   desc: optional text <=32_000
   ::: 
   ```

### `/homes/{home_id}/perms`
 
 - GET Response (**`home.user_id` only**)
   ```muon
   :::
   perm:
     user_id: int >0 <=1_200
     role_id: int >0 <=16
     actions: record
       cook: bool
       edit: bool
       view: bool
       vote: bool
   :::
   ```

### `/homes/{home_id}/perms/{user_id: int >0 <=1_200}`

 - PUT Request (**`home.user_id` only**)
   ```muon
   :::
   role_id: int >0 <=16
   :::
   ```

### `/homes/{home_id}/roles`

 - GET Response (**`home.user_id` only**)
   ```muon
   :::
   role: list record
     id: int >0 <=16
     name: text >=3 <=32
     actions: list choice
       cook
       edit
       view
       vote
   :::
   ```
 - POST Request (**`home.user_id` only**)
   ```muon
   :::
   name: text >=3 <=32
   actions: list choice
     cook
     edit
     view
     vote
   :::
   ```

### `/homes/{home_id}/roles/{role_id: int >0 <=16}`

 - DELETE (**`home.user_id` only**)
 - PATCH Request (**`home.user_id` only**)
   ```muon
   :::
   name: optional text >=3 <=32
   actions: optional list choice
     cook
     edit
     view
     vote
   :::
   ```

### `/homes/{home_id}/votes`

 - GET Response (action: **view**)
   ```muon
   :::
   vote: list record
     meal_id: int >0 <=1_200
     user_id: int >0 <=1_200
   :::
   ```

### `/homes/{home_id}/votes/{meal_id: int >0 <=1_200}`

 - PUT Request (action: **vote**)
   ```muon
   :::
   has_vote: bool
   :::
   ```

### `/users`

 - POST Request (no permissions required, use email verification)
   ```muon
   :::
   name: text >0 <=64
   email: text >=6 <=254
   :::
   ```

### `/users/{user_id: int >0 <=1_200}`

 - DELETE Request (no permissions required, use email verification)
   ```muon
   :::
   email: text >=6 <=254
   :::
   ```
 - PATCH Request (allowed after logged in, no additional permissions required)
   ```muon
   :::
   name: optional text >0 <=64
   email: optional text >=6 <=254
   :::
   ```
