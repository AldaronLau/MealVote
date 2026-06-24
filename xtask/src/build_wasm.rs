use std::{env, process::Command};

use clot::Opts;
use yansi::Paint;

fn compile() {
    let status = Command::new("cargo")
        .env(
            "RUSTFLAGS",
            "--remap-path-prefix=$PWD=_ \
             --remap-path-prefix=$HOME/.local/lib/cargo=- \
             --remap-path-prefix=$HOME/.local/lib/rustup=+ \
             --remap-path-prefix=$HOME=%",
        )
        .args([
            "build",
            "-p",
            "ui",
            "--target",
            "wasm32-unknown-unknown",
            "--profile",
            "release-wasm",
        ])
        .status()
        .expect("Failed to build");
    assert!(status.success());
}

fn wasm_bindgen(file: &str, build: &str) {
    let status = Command::new("wasm-bindgen")
        .args([
            "--out-dir",
            build,
            "--target",
            "web",
            "--no-typescript",
            "--remove-name-section",
            "--remove-producers-section",
            "--omit-default-module-path",
            file,
        ])
        .status()
        .expect("Failed to build");
    assert!(status.success());
}

fn wasm_opt(file: &str) {
    let status = Command::new("wasm-opt")
        .args([file, "-o", file, "-Os"])
        .status()
        .expect("Failed to optimize");
    assert!(status.success());
}

fn wasm_strip(file: &str) {
    let status = Command::new("wasm-strip")
        .args([file])
        .status()
        .expect("Failed to strip");
    assert!(status.success());
}

pub fn build_wasm(_opts: &dyn Opts) {
    let target = concat!(env!("CARGO_MANIFEST_DIR"), "/../target");
    let build = concat!(env!("CARGO_MANIFEST_DIR"), "/../build");
    let intermediate_file = format!("{build}/ui_bg.wasm");

    // Create WASM file with cargo
    eprintln!("    {} ui", "Building".green().bold());
    compile();
    // Run wasm-bindgen
    eprintln!("     {} ui", "Binding".green().bold());
    wasm_bindgen(
        &format!("{target}/wasm32-unknown-unknown/release/ui.wasm"),
        build,
    );
    // Run wasm-opt
    eprintln!("  {} ui", "Optimizing".green().bold());
    wasm_opt(&intermediate_file);
    // Run wasm-strip
    eprintln!("   {} ui", "Stripping".green().bold());
    wasm_strip(&intermediate_file);
    eprintln!(
        "       {} Wasm generated in `build`",
        "Done!".green().bold()
    );
    crate::build_html::build_html(_opts);
}
