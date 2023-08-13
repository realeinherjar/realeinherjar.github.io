#![allow(non_snake_case)]

use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

use components::{
    blog::{Blog, BlogPost},
    home::Home,
    notfound::NotFound,
};
use hooks::theme::Theme;

mod components {
    pub mod blog;
    pub mod content;
    pub mod home;
    pub mod nav;
    pub mod notfound;
}

mod hooks {
    pub mod markdown;
    pub mod theme;
}

mod utils {
    pub mod files;
    pub mod rss;
}

const URL: &str = "https://einhjerjar.github.io";
const AUTHOR: &str = "Einhjerjar";

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Theme::Dark);
    // toggle dark theme
    let _ = js_sys::eval("document.documentElement.classList.add('dark');");
    render! { Router::<Route> {} }
}

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[nest("/blog")]
        #[layout(Blog)]
            #[route("/")]
            Blog {},
            #[route("/blog/:title")]
            BlogPost { title: String, content: String },
        #[end_layout]
    #[end_nest]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
