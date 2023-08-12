use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::content::Markdown;
use crate::components::nav::Nav;
use crate::utils::files::list_md_files;
use crate::Route;

pub fn Blog(cx: Scope) -> Element {
    let content = include_str!("../markdown/test.md");
    render! {
        section {
            class: "font-sans antialiased leading-normal tracking-wider bg-cover bg-white dark:bg-gray-600",
            Nav {}
            div {
                class: "md:flex md:justify-center max-w-3xl mx-auto mb-4 bg-white dark:bg-gray-600 a-link-text-blue",
                Link {
                    to: Route::BlogPost {title: "test".into(), content: content.into()},
                    "Test"
                }
            }
        }
    }
}

#[inline_props]
pub fn BlogPost(cx: Scope, title: String, content: String) -> Element {
    render! {
        h2 { "Blog Post: {title}"}
        Markdown { class: "dark:text-white", content: content }
    }
}
