mod build_html;
mod build_wasm;

use clot::{Clot, Opts};

fn main() {
    Clot::new("Cargo Xtask")
        .cmd("build-wasm", build_wasm)
        .cmd("build-html", build_html)
        .execute()
}

fn build_wasm() -> Clot<impl Opts> {
    Clot::new("Build the WebAssembly UI").run(build_wasm::build_wasm)
}

fn build_html() -> Clot<impl Opts> {
    Clot::new("Build and layout the HTML").run(build_html::build_html)
}
