use dioxus::prelude::*;

pub fn Title(cx: Scope) -> Element {
    // でかでかとしたSteel Borne
    cx.render(rsx!(
        header {
            class: "text-slate-100 text-8xl bg-zinc-600",
            div { class: "container mx-auto flex flex-wrap p-0 flex-col md:flex-row justify-between items-center",
                h1 {
                    "Steel Borne"
                }
            }
        }
    ))
}
