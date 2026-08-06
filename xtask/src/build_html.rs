use std::fs;

use clot::Opts;

pub fn build_html(_opts: &dyn Opts) {
    let tree = pages::build_html();

    let res = concat!(env!("CARGO_MANIFEST_DIR"), "/../res");
    let build = concat!(env!("CARGO_MANIFEST_DIR"), "/../build");

    fs::write(format!("{build}/index.html"), String::from(tree)).unwrap();
    fs::copy(format!("{res}/icon.svg"), format!("{build}/icon.svg")).unwrap();
}
