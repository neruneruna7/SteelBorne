use crate::Route;
use dioxus::prelude::*;
use dioxus_router::components::Link;

#[derive(PartialEq, Props, Clone)]
pub struct NavigationBarProps {
    now_route: Route
}

pub fn NavigationBar(cx: Scope<NavigationBarProps>) -> Element {
    let now_route = &cx.props.now_route;
    // let navigate_to = &;
    cx.render(rsx!(
        div {
            class: "absolute top-0 left-0 w-2/5 h-full border pt-5 pl-6 z-10 flex flex-col justify-start items-stretch",
            div {
                // 青っぽいグラデーション付きで，現在の場所を示す
                class: "bg-gradient-to-r from-slate-600 w-auto pl-4 flex flex-wrap flex-col items-start",
                h1 {
                    class: "text-slate-100 text-2xl text-center",
                    match now_route {
                        Route::Home => "HOME",
                        Route::Profile => "PROFILE",
                    }
                }
            }
            div {
                // ナビゲーションするリンクを囲む
                // 左側にだけ縦線が少しはいる
                class: "w-auto h-1/2 ml-2 border-l-2 border-gray-600 mt-5 flex flex-col justify-start items-stretch",
                NavigationLink { navigate_to: Route::Home {} }
                NavigationLink { navigate_to: Route::Profile {} }
            }
        }
    ))
}


#[derive(PartialEq, Props, Clone)]
pub struct NavigationLinkProps {
    navigate_to: Route
}

fn NavigationLink(cx: Scope<NavigationLinkProps>) -> Element {
    let navigate_to = cx.props.navigate_to.clone();

    cx.render(rsx!(
        Link {
            to: navigate_to,
            class: "mb-2",
            div {
                class: "w-full p-1 hover:bg-gray-800",
                h1 {
                    class: "text-slate-100  pl-5 text-2xl text-left",
                    cx.props.navigate_to.route_name()
                }
            }
        }
    ))
}