use dioxus::prelude::*;

use crate::components::{Counter, Selector, Title};
use crate::Route;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "relative z-0 bg-neutral-950 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Title {}
            Selector { text:"PROFILE".to_string(), navigate_to: Route::Profile {} }
            // Selector { text:"LANGUAGE".to_string() }
            Counter {}
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
            }
        }
    })
}
