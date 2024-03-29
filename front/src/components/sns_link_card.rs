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
            class: "flex space-x-4",
            a {
                class: "w-8 h-8",
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