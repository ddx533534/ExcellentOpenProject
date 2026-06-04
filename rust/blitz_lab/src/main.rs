// src/main.rs
use dioxus_native::prelude::*; // Use native prelude instead of dioxus::prelude::*

fn main() {
    // Launch using the native backend directly
    dioxus_native::launch(app);
}

fn app() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 100vh; background-color: #232323;",
            h1 { style: "color: white; font-size: 32px;", "Hello Android!" }
            p { style: "color: #b3b3b3;", "Powered natively by Blitz & Vello" }
        }
    }
}