use crate::Route;
use dioxus::prelude::*;

use crate::components::{NavigationBar, ProfileCard};

pub fn Profile(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "relative z-0 bg-neutral-950 w-screen h-auto min-h-screen flex flex-row justify-start items-stretch",
            NavigationBar { now_route: Route::Profile {} }
            ProfileCard{icon_image_path: String::from("user_icon.jpg")}
        }

    })
}
