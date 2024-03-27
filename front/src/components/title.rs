use dioxus::prelude::*;

pub fn Title(cx: Scope) -> Element {
    // でかでかとしたSteel Borne
    cx.render(rsx!(
            div { class: "bg-zinc-600 w-full py-24 flex flex-wrap flex-col md:flex-row justify-center items-center",
                h1 {
                    class: "text-slate-100 text-8xl text-center",
                    "Steel Borne"
                }
            }
        
    ))
}
