use crate::Route;
use dioxus::prelude::*;
use dioxus_router::components::Link;

#[derive(PartialEq, Props, Clone)]
pub struct ProfileCardProps {
    icon_image_path: String,
}

#[component]
pub fn ProfileCard(cx: Scope<ProfileCardProps>) -> Element {
    cx.render(rsx!(
        div {
            class: "border-2 border-gray-600",
        }
    ))
}


#[derive(PartialEq, Props, Clone)]
pub struct NameProps {
    name: String,
}

#[component]

fn Name(cx: Scope<ProfileCardProps>) -> Element {
    cx.render(rsx!(
        div {
            class: "text-slate-100 ",
        }
    ))
}