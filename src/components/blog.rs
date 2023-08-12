use dioxus::prelude::*;

use crate::components::nav::Nav;
use crate::hooks::markdown::use_markdown;

pub fn Blog(cx: Scope) -> Element {
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
                        p {class: "pt-8 text-sm", "Totally optional short description about yourself, what you do and so on."}
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
