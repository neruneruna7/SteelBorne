use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SnsLinkCardProps {
    sns_url: &'static str,
    icon_path: &'static str,
    alt: &'static str,
}

pub fn SnsLinkCard(cx: Scope<SnsLinkCardProps>) -> Element {
    let sns_url = cx.props.sns_url;
    let icon_path = cx.props.icon_path;
    let alt = cx.props.alt;

    cx.render(rsx!(
        div {
            class: "w-fit flex space-x-4 m-4 border-gray-300 border-2 bg-gray-500",
            a {
                class: "w-12 h-12 p-1",
                href: sns_url,
                img {
                    class: "w-full h-full",
                    src: icon_path,
                    alt: alt,
                }
            }
        }
    ))
}