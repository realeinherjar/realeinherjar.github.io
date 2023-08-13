use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::hooks::markdown::use_markdown;
use crate::Route;

#[inline_props]
pub fn Href<'a>(cx: Scope, to: &'a str, children: Element<'a>) -> Element {
    cx.render(rsx! {
        a { class: "text-cyan-700 dark:text-cyan-100 underline", href: "{to}", target: "_blank", children }
    })
}

#[inline_props]
pub fn Markdown<'a>(cx: Scope, class: &'a str, content: String) -> Element {
    let md_parser = use_markdown(&cx);
    let content = md_parser(content.into());
    let extra_class = &class;
    render! {
        div { id: "markdown-body", class: "prose {extra_class}", dangerous_inner_html: "{content}" }
    }
}
