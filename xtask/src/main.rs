mod build_wasm;

use clot::{Clot, Opts};

fn main() {
    Clot::new("Cargo Xtask")
        .cmd("build-wasm", build_wasm)
        .execute()
}

fn build_wasm() -> Clot<impl Opts> {
    Clot::new("Build the WebAssembly UI").run(build_wasm::build_wasm)
}
