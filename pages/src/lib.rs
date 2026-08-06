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

    head.meta().charset("utf-8").close();
    head.meta()
        .name("viewport")
        .content("width=device-width, initial-scale=1")
        .close();
    head.title_el().cdata("MealVote").close();
    head.base().href("").close();
    head.link().rel("icon").href("icon.svg").close();

    // Set up style
    let mut css = String::new();

    write!(
        &mut css,
        "{}",
        Rule::new(Sel::tp("html"), Prop::new().background_color("#213141"))
    )
    .ok();

    head.style_el().cdata(css).close();
    head.close();

    // Set up visible page
    let mut body = html.body();

    // Wasm linking
    body.script()
        .r#type("module")
        .cdata(r#"import init, {} from './ui.js';init("./ui_bg.wasm");"#)
        .close();

    let _main = body.main();

    body.close();
    html.close();
    tree
}
