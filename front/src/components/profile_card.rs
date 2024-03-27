use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ProfileCardProps {
    icon_image_path: String,
}

#[component]
pub fn ProfileCard(cx: Scope<ProfileCardProps>) -> Element {
    cx.render(rsx!(
        div {
            class: "border-2 border-gray-600, w-full p-10",
            div {
                class: "border-2 w-full h-full p-7 bg-slate-900",
                Name {name: String::from("621")}
                ProfileImage {icon_image_path: cx.props.icon_image_path.clone()}
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
                class: "text-slate-100 text-lg text-start",
                "Name"
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
            class: "w-1/2 h-auto m-4 border-black border-4  overflow-hidden",
            img {
                class: "w-auto h-auto",
                alt: "profile image",
                src: image_path,
                "loading": "lazy"
            }
        }
    ))
}
