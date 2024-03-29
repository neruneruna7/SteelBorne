use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ProfileCardProps {
    icon_image_path: String,
}

use super::sns_link_card::SnsLinkCard;

#[component]
pub fn ProfileCard(cx: Scope<ProfileCardProps>) -> Element {
    cx.render(rsx!(
        div {
            class: "border-2 border-gray-600, w-full p-10",
            div {
                class: "border-2 w-full h-full p-7 bg-slate-600/50",
                Name {name: String::from("neruneruna7")}
                ProfileImage {icon_image_path: cx.props.icon_image_path.clone()}
                SnsLinkCard { sns_url: "https://github.com/neruneruna7", icon_path: "sns_icons/github.svg", alt: "github icon" }
            }
        }
    ))
}

#[derive(PartialEq, Props, Clone)]
pub struct NameProps {
    name: String,
}

#[component]
fn Name(cx: Scope<NameProps>) -> Element {
    cx.render(rsx!(
        div {
            class: "w-full p-1",
            p {
                class: "text-slate-100 text-base text-start",
                "■ NAME"
            }
            h1 {
                class: "text-slate-100 text-2xl text-start",
                cx.props.name.clone()
            }
        }
    ))
}

#[derive(PartialEq, Props, Clone)]
pub struct ProfileImageProps {
    icon_image_path: String,
}

#[component]
pub fn ProfileImage(cx: Scope<ProfileImageProps>) -> Element {
    let image_path = cx.props.icon_image_path.as_str();
    cx.render(rsx!(
        div {
            class: "w-auto h-auto m-  overflow-auto",
            img {
                class: "max-h-52 h-1/3 w-auto border-gray-700 border-4",
                alt: "profile image",
                src: image_path,
                "loading": "lazy"
            }
        }
    ))
}
