use dioxus::prelude::*;
use views::{Home, Navbar, Stats};

pub mod api;
mod components;
mod views;

#[cfg(feature = "server")]
pub mod server;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    
    #[layout(Navbar)]

        #[route("/")]
        Home {},

        #[route("/stats/:name")]
        Stats { name: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {},
    }
}
