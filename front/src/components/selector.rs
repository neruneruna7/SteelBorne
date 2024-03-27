use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SelectorProps {
    text: String,
}

pub fn Selector(cx: Scope<SelectorProps>) -> Element {
    let text = &cx.props.text;
    cx.render(rsx!(
            div { class: "bg-inherit  w-full my-4 px-2 flex flex-wrap flex-col md:flex-row justify-center items-center",
                div {
                    class: "w-48 p-1 hover:bg-gradient-to-r from-neutral-950 from-0% via-neutral-700 via-50% to-neutral-950",
                    h1 {
                        class: "text-slate-100 text-2xl text-center",
                        "{text}"
                    }
                }
            }
        
    ))
}
