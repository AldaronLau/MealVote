use std::{fmt::Write, fs};

use clot::Opts;
use hatmil::{
    Tree,
    css::{Prop, Rule, Sel},
};

pub fn build_html(_opts: &dyn Opts) {
    // Set up boilerplate
    let mut tree = Tree::new();
    let mut html = tree.html();
    let mut head = html.head();

    head.meta().charset("utf-8");
    head.meta()
        .name("viewport")
        .content("width=device-width, initial-scale=1");
    head.title("MealVote");
    head.base().href("");
    head.link().rel("icon").href("icon.svg");

    // Set up style
    let mut css = String::new();

    write!(
        &mut css,
        "{}",
        Rule::new(Sel::tp("html"), Prop::new().background_color("#213141"))
    )
    .ok();

    head.style_el().cdata(css).close();

    // Set up visible page
    let mut body = html.body();
    let mut main = body.main();

    // Wasm linking
    main.script()
        .r#type("module")
        .cdata(r#"import init, {} from './ui.js';init("./ui_bg.wasm");"#);

    let res = concat!(env!("CARGO_MANIFEST_DIR"), "/../res");
    let build = concat!(env!("CARGO_MANIFEST_DIR"), "/../build");

    fs::write(format!("{build}/index.html"), String::from(tree)).unwrap();
    fs::copy(format!("{res}/icon.svg"), format!("{build}/icon.svg")).unwrap();
}
