use std::fmt::Write;

use hatmil::{
    Tree,
    css::{Prop, Rule, Sel},
};

/// Return HTML tree for the UI.
pub fn build_html() -> Tree {
    // Set up boilerplate
    let mut tree = Tree::new();
    let mut html = tree.html();
    let mut head = html.head();

    head.meta().charset("utf-8");
    head.meta()
        .name("viewport")
        .content("width=device-width, initial-scale=1");
    head.title_el().cdata("MealVote").close();
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

    // Wasm linking
    body.script()
        .r#type("module")
        .cdata(r#"import init, {} from './ui.js';init("./ui_bg.wasm");"#);

    let _main = body.main();

    tree
}
