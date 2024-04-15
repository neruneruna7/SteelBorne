#![allow(non_snake_case)]
mod components;
mod pages;

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn main() {
    // wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    // launch the web app
    dioxus_web::launch(App);
}

use pages::home::Home;
use pages::profile::Profile;

#[rustfmt::skip]
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home,
    #[route("/profile")]
    Profile,
    // #[route("/language")]
    // Language,
}

impl Route {
    // DisplayトレイトはRoutableと衝突したので実装できない
    pub fn route_name(&self) -> String {
        match self {
            Route::Home => String::from("HOME"),
            Route::Profile => String::from("PROFILE"),
        }
    }
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx!(Router::<Route> {}))
}
