#![allow(non_snake_case)]
mod components;

use components::{Footer, Header, Title};
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        main {
            class: "relative z-0 bg-neutral-500 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {}
            Title {}
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
            }
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    })
}
