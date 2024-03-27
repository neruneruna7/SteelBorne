use crate::Route;
use dioxus::prelude::*;

use crate::components::{Counter, Selector, Title, NavigationBar};

pub fn Profile(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        main {
            class: "relative z-0 bg-neutral-950 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            NavigationBar { now_route: Route::Profile {} }

            Title {}
            Selector { text:"HOME".to_string(), navigate_to: Route::Home {} }
            h1 {
                class: "text-slate-100 text-2xl text-center",
                "here is Profile"
            }
            // Selector { text:"LANGUAGE".to_string() }
            Counter {}
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
            }
        }

    })
}
