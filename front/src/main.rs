#![allow(non_snake_case)]
mod components;
mod pages;

use components::{Counter, Footer, Header, Selector, Title};
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
    // 引数で渡すとうまくいかないので，冗長かもしれんがこのコードを使う
    pub fn routing(&self) -> Self {
        match self {
            Route::Home => Route::Home {},
            Route::Profile => Route::Home {},
        }
    }
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx!(Router::<Route> {}))
}
