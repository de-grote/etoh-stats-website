use crate::components::Echo;
use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }

        Echo {}
    }
}
