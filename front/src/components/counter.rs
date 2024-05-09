use dioxus::prelude::*;

pub fn Counter(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    cx.render(
        rsx!(
            div { class: "bg-inherit  w-full my-4 px-2 flex flex-wrap flex-col md:flex-row justify-center items-center",
                div {
                    class: "w-48 p-1 hover:bg-gradient-to-r from-neutral-950 from-0% via-neutral-700 via-50% to-neutral-950 flex flex-wrap flex-col md:flex-row justify-center items-center",
                    button {                    
                        class: "text-slate-100 text-2xl text-center",
                        onclick: move |_| count += 1, 
                        "Up high!: {count}" 
                    }
                }
            }
            div { class: "bg-inherit  w-full my-4 px-2 flex flex-wrap flex-col md:flex-row justify-center items-center",
            div {
                class: "w-48 p-1 hover:bg-gradient-to-r from-neutral-950 from-0% via-neutral-700 via-50% to-neutral-950 flex flex-wrap flex-col md:flex-row justify-center items-center",
                button {                    
                    class: "text-slate-100 text-2xl text-center",
                    onclick: move |_| count -= 1, 
                    "Down low!: {count}" 
                }
            }
        }
        )            
    )
}
