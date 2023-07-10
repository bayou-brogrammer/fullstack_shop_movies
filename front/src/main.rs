#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
mod components;

use components::{Footer, Header};
use dioxus::prelude::*;

fn main() {
    // wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));

    // Launch the web application using the App component as the root.
    // dioxus_web::launch(App);
    dioxus_desktop::launch(App);
}

// Define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {},
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
            }, 
            Footer {}
        }
    })
}
