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
        section {
            class: "font-sans antialiased leading-normal tracking-wider bg-cover bg-white dark:bg-gray-600 dark:text-white",
            Nav {}
            div { class: "max-w-4xl flex items-center h-auto lg:h-screen flex-wrap mx-auto my-32 lg:my-0",
                div {
                    id: "profile",
                    class: "w-full lg:w-3/5 rounded-lg lg:rounded-l-lg lg:rounded-r-none shadow-2xl bg-white dark:bg-gray-600 mx-6 lg:mx-0",
                    div {
                        class: "p-4 md:p-12 text-center lg:text-left",
                        div {class: "block lg:hidden rounded-full shadow-2xl mx-auto -mt-16 h-48 w-48 bg-cover bg-center", style: "background-image: url('images/profile.png')"} // image for mobile view
                        h1 {class: "text-3xl font-bold pt-6 lg:pt-0", "Einherjar"}
                        div {class: "mx-auto lg:mx-0 w-4/5 pt-3 border-b-2 opacity-25"}
                        p {
                            class: "pt-8 text-lg",
                            "A toxic Bitcoin-maximalist and Rust/Python developer"
                        }
                        h1 { class: "text-3xl font-bold pt-6 lg:pt-0", "Einherjar" }
                        div { class: "mx-auto lg:mx-0 w-4/5 pt-3 border-b-2 opacity-25" }
                        p { class: "pt-8 text-lg", "A toxic Bitcoin-maximalist and Rust/Python developer" }
                        div {
                            class: "mt-6 pb-16 pb-0 w-4/5 lg:w-full mx-auto flex flex-wrap items-center",
                            id: "social-icons",
                            Link {
                                class: "block py-2 pl-3 pr-4 text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                                to: "https://github.com/realeinherjar",
                                Icon { height: 30, width: 30, icon: FaGithub }
                            }
                            Link {
                                class: "block py-2 pl-3 pr-4 text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                                to: "https://primal.net/p/npub1mcgkta7n5ptnha34acasmld7z3psp6lwlmqgqwzp9c4jevnv25lqm02agr",
                                Icon { height: 30, width: 30, icon: FaBullhorn }
                            }
                            Link {
                                class: "block py-2 pl-3 pr-4 text-black dark:text-white hover:text-gray-800 dark:hover:text-gray-200",
                                to: "mailto:realeinherjar@proton.me",
                                Icon { height: 30, width: 30, icon: FaEnvelope }
                            }
                        }
                        br {}
                        h2 { class: "text-2xl font-bold pt-6 lg:pt-0", "Tools" }
                        div { class: "mx-auto lg:mx-0 w-4/5 pt-2 border-b-2 opacity-25" }
                        div {
                            class: "mt-6 pb-16 pb-0 w-4/5 lg:w-full mx-auto flex flex-wrap items-center",
                            id: "tool-icons",
                            a { href: "https://www.bitcoin.org",
                                img {
                                    src: "https://upload.wikimedia.org/wikipedia/commons/4/46/Bitcoin.svg",
                                    alt: "bitcoin",
                                    width: "30",
                                    height: "30"
                                }
                            }
                            a { href: "https://www.lightning.network",
                                img {
                                    src: "https://upload.wikimedia.org/wikipedia/commons/5/5a/Lightning_Network.svg",
                                    alt: "lightning",
                                    width: "30",
                                    height: "30"
                                }
                            }
                            a { href: "https://www.rust-lang.org",
                                img {
                                    src: "https://raw.githubusercontent.com/devicons/devicon/master/icons/rust/rust-plain.svg",
                                    alt: "rust",
                                    width: "30",
                                    height: "30"
                                }
                            }
                            a { href: "https://www.python.org",
                                img {
                                    src: "https://raw.githubusercontent.com/devicons/devicon/master/icons/python/python-original.svg",
                                    alt: "python",
                                    width: "30",
                                    height: "30"
                                }
                            }
                            a { href: "https://www.cprogramming.com/",
                                img {
                                    src: "https://raw.githubusercontent.com/devicons/devicon/master/icons/c/c-original.svg",
                                    alt: "c",
                                    width: "30",
                                    height: "30"
                                }
                            }
                            a { href: "https://www.w3schools.com/cpp/",
                                img {
                                    src: "https://raw.githubusercontent.com/devicons/devicon/master/icons/cplusplus/cplusplus-original.svg",
                                    alt: "cplusplus",
                                    width: "30",
                                    height: "30"
                                }
                            }
                            a { href: "https://www.bitcoindevkit.org",
                                img {
                                    src: "https://avatars.githubusercontent.com/u/62867074?s: 200&v: 4",
                                    alt: "bdk",
                                    width: "30",
                                    height: "30"
                                }
                            }
                            a { href: "https://www.lightningdevkit.org",
                                img {
                                    src: "https://avatars.githubusercontent.com/u/60484029?s: 200&v: 4",
                                    alt: "ldk",
                                    width: "30",
                                    height: "30"
                                }
                            }
                            a { href: "https://www.dioxuslabs.com",
                                img {
                                    src: "https://avatars.githubusercontent.com/u/79236386?s: 200&v: 4",
                                    alt: "ldk",
                                    width: "30",
                                    height: "30"
                                }
                            }
                            a { href: "https://www.nixos.org/",
                                img {
                                    src: "https://github.com/NixOS/nixos-artwork/raw/master/logo/nix-snowflake.svg",
                                    alt: "nixos",
                                    width: "30",
                                    height: "30"
                                }
                            }
                            a { href: "https://www.docker.com/",
                                img {
                                    src: "https://raw.githubusercontent.com/devicons/devicon/master/icons/docker/docker-original-wordmark.svg",
                                    alt: "docker",
                                    width: "30",
                                    height: "30"
                                }
                            }
                        }
                    }
                }
                div { class: "w-full lg:w-2/5",
                    img {
                        class: "rounded-none lg:rounded-lg shadow-2xl hidden lg:block",
                        src: "images/profile.png"
                    }
                }
            }
        }
    }
}
