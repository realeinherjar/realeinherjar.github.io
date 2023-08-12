use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{
        fa_brands_icons::FaGithub,
        fa_solid_icons::{FaBullhorn, FaEnvelope},
    },
    Icon,
};
use dioxus_router::prelude::*;

use crate::components::nav::Nav;

pub fn Home(cx: Scope) -> Element {
    render! {
        head {
            link {
                rel: "stylesheet",
                href: "/public/tailwind.css"
            }
        }
        section {
            class: "font-sans antialiased leading-normal tracking-wider bg-cover bg-white dark:bg-gray-600",
            Nav {}
            div {
                class: "max-w-4xl flex items-center h-auto lg:h-screen flex-wrap mx-auto my-32 lg:my-0",
                div { // main column
                    id: "profile",
                    class: "w-full lg:w-3/5 rounded-lg lg:rounded-l-lg lg:rounded-r-none shadow-2xl bg-white dark:bg-gray-600 mx-6 lg:mx-0",
                    div {
                        class: "p-4 md:p-12 text-center lg:text-left",
                        div {class: "block lg:hidden rounded-full shadow-xl mx-auto -mt-16 h-48 w-48 bg-cover bg-center", style: "background-image: url('images/profile.png')"} // image for mobile view
                        h1 {class: "text-3xl font-bold pt-8 lg:pt-0 hover:text-accent", "Einherjar"}
                        div {class: "mx-auto lg:mx-0 w-4/5 pt-3 border-b-2 opacity-25"}
                        p {
                            class: "pt-8 text-sm",
                            "A toxic Bitcoin-maximalist and Rust/Python developer"
                        }
                        div { // icons
                            class: "mt-6 pb-16 lg:pb-0 w-4/5 lg:w-full mx-auto flex flex-wrap items-center",
                            Link {
                                class: "block py-2 pl-3 pr-4 text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                                to: "https://github.com/realeinherjar",
                                Icon {height: 24, width: 24, icon: FaGithub}
                            }
                            Link {
                                class: "block py-2 pl-3 pr-4 text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                                to: "https://primal.net/p/npub1mcgkta7n5ptnha34acasmld7z3psp6lwlmqgqwzp9c4jevnv25lqm02agr",
                                Icon {height: 24, width: 24, icon: FaBullhorn}
                            }
                            Link {
                                class: "block py-2 pl-3 pr-4 text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                                to: "mailto:realeinherjar@proton.me",
                                Icon {height: 24, width: 24, icon: FaEnvelope}
                            }

                        }
                    }
                }
                div {  // image column
                    class: "w-full lg:w-2/5",
                    img {
                        class: "rounded-none lg:rounded-lg shadow-2xl hidden lg:block",
                        src: "images/profile.png",
                    }
                }
            }
        }
    }
}
