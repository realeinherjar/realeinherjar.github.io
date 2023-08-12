use dioxus::prelude::*;

#[inline_props]
pub fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}
