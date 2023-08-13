use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::content::Markdown;
use crate::Route;

#[inline_props]
pub fn Blog(cx: Scope) -> Element {
    render! {
        Outlet::<Route> {}
    }
}

#[inline_props]
pub fn BlogList(cx: Scope) -> Element {
    let content = include_str!("../markdown/2023-08-13-hello.md").to_string();
    render! {
        section { class: "font-sans antialiased leading-normal tracking-wider bg-cover bg-white dark:bg-gray-600 dark:text-white",
            div { class: "max-w-4xl flex items-center h-auto lg:h-screen flex-wrap mx-auto my-32 lg:my-0",
                div {
                    id: "blog-list",
                    class: "w-full rounded-lg lg:rounded-l-lg lg:rounded-r-none shadow-2xl mx-6 lg:mx-0",
                    h1 { class: "text-3xl font-bold pt-6 lg:pt-0", "Blog" }
                    div { class: "mx-auto lg:mx-0 w-4/5 pt-3 border-b-2 opacity-25" }
                     ul { class: "pt-8 text-lg",
                        li {
                            Link {
                                to: Route::BlogPost {
                                    name: "2023-08-13 - Hello World".to_string(),
                                    content:  include_str!("../markdown/2023-08-13-hello.md").to_string(),
                                },
                                "2023-08-13 - Hello World"
                            }
                        }
                    }
                    Markdown {class: "inherit-text", content: content.into() }
                }
            }
        }
    }
}

#[inline_props]
pub fn BlogPost(cx: Scope, name: String, content: String) -> Element {
    render! {
        section { class: "font-sans antialiased leading-normal tracking-wider bg-cover bg-white dark:bg-gray-600 dark:text-white",
            div { class: "max-w-4xl flex items-center h-auto lg:h-screen flex-wrap mx-auto my-32 lg:my-0",
                h2 { "Blog Post: {name}" }
                div {
                    id: "blog-post",
                    class: "w-full rounded-lg lg:rounded-l-lg lg:rounded-r-none shadow-2xl mx-6 lg:mx-0",
                    Markdown {class: "inherit-text", content: content.into() }
                }
            }
        }
    }
}
