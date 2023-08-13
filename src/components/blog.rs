use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(PartialEq, Eq)]
pub struct BlogPost {
    pub category: &'static str,
    pub date: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub link: &'static str,
    pub content: &'static str,
}

#[inline_props]
pub fn SinglePost(cx: Scope, post: BlogPost) -> Element {
    let BlogPost { content, .. } = post;

    render! {
        section { class: "font-sans overflow-hidden bg-white dark:bg-gray-600 dark:text-white",
            div { class: "container lg:px-20 xl:px-48 pt-12 pb-12 mx-auto",
                style {
                    ".markdown-body h1 {{ font-size: 2.125rem; font-weight: bold; }}"
                    ".markdown-body h2 {{ font-size: 1.875rem; font-weight: bold; }}"
                    ".markdown-body ul {{ list-style: disc; }}"
                    ".markdown-body li {{ display: list-item; }}"
                    ".markdown-body img {{ max-height: 500px; margin-left: auto; margin-right: auto; padding-left: 4px; padding-right: 4px; }}"
                    ".markdown-body a {{ color: #5C6BC0; }}"
                }
                article { class: "markdown-body space-y-4", dangerous_inner_html: format_args!("{}", content) }
            }
        }
    }
}

pub const POST_HELLO: BlogPost = BlogPost {
    category: "Tech",
    date: "2023-08-13",
    title: "Hello World",
    description: "How I created this website",
    link: "/blog/hello/",
    content: include_str!("../../posts/2023-08-13-hello.html"),
};

#[inline_props]
pub fn PostHello(cx: Scope) -> Element {
    render! { SinglePost { post: POST_HELLO } }
}

pub const POSTS: &[BlogPost] = &[POST_HELLO];

#[inline_props]
pub fn BlogList(cx: Scope) -> Element {
    render! {
        section { class: "body-font overflow-hidden dark:bg-ideblack",
            div { class: "container max-w-screen-lg pt-12 pb-12 mx-auto",
                div { class: "-my-8 px-8 pb-12",
                    // Header
                    h2 { class: "dark:text-white mb-8 md:mb-16 sm:text-3xl text-2xl font-medium title-font font-sans",
                        "Recent Blog Posts"
                    }
                    section { class: "body-font overflow-hidden dark:bg-ideblack",
                        div { class: "container px-6 mx-auto",
                            div { class: "-my-8 divide-y-2 divide-gray-100",
                                POSTS.iter().map(|post| rsx! { BlogPostItem { post: post } })
                            }
                        }
                    }
                }
            }
        }
    }
}

#[inline_props]
fn BlogPostItem(cx: Scope, post: &'static BlogPost) -> Element {
    let BlogPost {
        category,
        date,
        title,
        description,
        link,
        ..
    } = post;

    render! {
        div { class: "py-8 flex flex-wrap md:flex-nowrap",
            div { class: "md:w-32 md:mb-0 mb-6 flex-shrink-0 flex flex-col",
                span { class: "font-semibold title-font text-gray-600 dark:text-white", "{category}" }
                span { class: "mt-1 text-gray-500 text-sm", "{date}" }
            }
            div { class: "pl-3 md:flex-grow",
                h2 { class: "text-2xl font-medium text-gray-900 title-font mb-2 dark:text-white",
                    "{title}"
                }
                p { class: "leading-relaxed dark:text-white text-base dark:opacity-75",
                    "{description}"
                }
                Link { class: "text-indigo-400 inline-flex items-center mt-4", to: *link, "Read more" }
            }
        }
    }
}
