#![allow(non_snake_case)]

use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

use components::{home::Home, notfound::NotFound};
use hooks::theme::Theme;

mod components {
    pub mod home;
    pub mod nav;
    pub mod notfound;
}

mod hooks {
    pub mod theme;
}

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
enum Route {
    #[route("/")]
    Home {},
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
