use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaCircleHalfStroke, FaHouse},
    Icon,
};
use dioxus_router::prelude::*;

use crate::hooks::theme::Theme;
use crate::Route;

pub fn Nav(cx: Scope) -> Element {
    let theme = use_shared_state::<Theme>(cx).unwrap();
    let current_theme = *theme.read();
    render! {
        nav { class: "flex flex-wrap items-center justify-between w-full py-4 md:py-0 px-4 antialiased leading-normal tracking-wider",
            div {
                class: "max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4",
                id: "navbar",
                Link {
                    class: "block py-2 pl-3 pr-4 text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                    to: Route::Home {},
                    Icon {height: 30, width: 30, icon: FaHouse}
                }
                a {
                    class: "block py-2 pl-3 pr-4 text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                    href: "javascript:;",
                    onclick: move |_| {
                        *theme
                            .write() = match current_theme {
                            Theme::Dark => {
                                let _ = js_sys::eval("document.documentElement.classList.remove('dark');");
                                Theme::Light
                            }
                            Theme::Light => {
                                let _ = js_sys::eval("document.documentElement.classList.add('dark');");
                                Theme::Dark
                            }
                        };
                    },
                    Icon { height: 30, width: 30, icon: FaCircleHalfStroke }
                }
            }
        }
    }
}
