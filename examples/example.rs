#![recursion_limit = "512"]

use tagz::*;

fn main() {
    let html = Html::lang("en")
        .child(
            Head::child(Title::child("😍"))
                .child(Script::src("script.js"))
                .child(Link::href("style.css")),
        )
        .child(
            Body::class("dark")
                .hidden(0.5)
                .draggable(true)
                .child(Heading1::id("hello-world").child("Hello, world!"))
                .child(
                    Division::child("It's ")
                        .child(Bold::child("blazingly"))
                        .child(" fast!"),
                )
                .child(
                    Anchor::data("href", "https://github.com/truchi/tagz")
                        .target("_blank")
                        .child(Span::child("Link")),
                )
                .child(Custom::new("my-element").id("custom")),
        );

    dbg!(&html);
    println!("{html}");
}
