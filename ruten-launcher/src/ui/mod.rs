use dioxus::prelude::*;
use dioxus::desktop::DesktopContext;
use dioxus_heroicons::{Icon, solid::Shape};

enum Tab {
    Home,
    Logs,
    Settings
}

pub fn app(cx: Scope) -> Element {
    let style = include_str!("./style.css");
    let tab = use_state(&cx, || Tab::Home);
    
    let window = cx.consume_context::<DesktopContext>().unwrap();
    
    cx.render(rsx! {
        style {
            "{style}"
        }
        div {
            nav {
                class: "sidebar",
                div {
                    style: "flex-grow: 2;",
                    button {
                        class: "btn btn-w-icon",
                        onclick: move |_| tab.set(Tab::Home),
                        Icon {
                            icon: Shape::Home,
                            fill: "var(--text)"
                        }
                        "Home"
                    }
                    button {
                        class: "btn btn-w-icon",
                        onclick: move |_| tab.set(Tab::Logs),
                        Icon {
                            icon: Shape::Newspaper,
                            fill: "var(--text)"
                        }
                        "Logs"
                    }
                    button {
                        class: "btn btn-w-icon",
                        onclick: move |_| tab.set(Tab::Settings),
                        Icon {
                            icon: Shape::Cog,
                            fill: "var(--text)"
                        }
                        "Settings"
                    }
                }
                div {
                    button {
                        class: "btn btn-primary",
                        style: "width: 10rem;",
                        "Launch"
                    }
                }
            }
            div {
                class: "content",
                match **tab {
                    Tab::Home => home(cx),
                    Tab::Logs => logs(cx),
                    Tab::Settings => settings(cx)
                }
            }
        }
    })
}

fn home(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Versions" }
        hr {}
        div {
            class: "version-scroll",
            button {
                class: "version-option",
                div {
                    div {
                        p {
                            "a"
                        }
                    }
                }
                img {
                    src: "https://i.imgur.com/YTiYCxj.png"
                }
            }
            button {
                class: "version-option",
                img {
                    src: "https://i.imgur.com/LY7VkYs.png"
                }
            }
            button {
                class: "version-option",
                img {
                    src: "https://i.imgur.com/FPN8SPz.png"
                }
            }
            button {
                class: "version-option",
                img {
                    src: "https://i.imgur.com/FPoujxP.png"
                }
            }
            button {
                class: "version-option",
                img {
                    src: "https://i.imgur.com/G3PlrtL.png"
                }
            }
            button {
                class: "version-option version-dummy",
                "+"
            }
        }
        hr {}
    })
}

fn logs(cx: Scope) -> Element {
    cx.render(rsx! {
        p {"Logs"}
    })
}

fn settings(cx: Scope) -> Element {
    cx.render(rsx! {
        p {"Settings"}
    })
}