use dioxus::prelude::*;

#[component]
pub fn Scrollable(children: Element, height: i32) -> Element {
    rsx! {
        div {
            class: "scrollable",
            style: "height: {height}px",
            { children }
        }
    }
}