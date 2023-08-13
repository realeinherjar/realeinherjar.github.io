#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;

use hooks::theme::Theme;

use utils::rss::write_rss_file;

use components::{
    blog::{BlogList, PostHello},
    home::Home,
    nav::Nav,
    notfound::NotFound,
};

mod components {
    pub mod blog;
    pub mod home;
    pub mod nav;
    pub mod notfound;
}

mod hooks {
    pub mod theme;
}

mod utils {
    pub mod rss;
}

const URL: &str = "https://realeinherjar.github.io";
const AUTHOR: &str = "Einherjar";

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();
    log::info!("starting app");

    // Launch App
    dioxus_web::launch(app);

    // RSS Feed
    write_rss_file();
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Theme::Dark);
    // toggle dark theme
    let _ = js_sys::eval("document.documentElement.classList.add('dark');");
    render! {
        div { class: "bg-white dark:bg-gray-600 dark:text-white", Router::<Route> {} }
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    // All routes under the NavBar layout will be rendered inside of the NavBar Outlet
    #[layout(Nav)]
        #[route("/")]
        Home {},
        #[nest("/blog")]
            #[route("/")]
            BlogList {},
            #[route("/hello")]
            PostHello {},
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
