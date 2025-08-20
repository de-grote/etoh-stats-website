use dioxus::prelude::*;

#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    let goto_page = move || {
        let res = response();
        if res.len() >= 3 {
            navigator().push(crate::Route::Stats { name: res });
        }
    };

    rsx! {
        div {
            id: "echo",
            h4 { "Check EToH Stats" }
            input {
                placeholder: "Roblox Username",

                oninput: move |event| {
                    use_effect(move || {
                        response.set(event.value());
                    });
                },

                onkeypress: move |event| {
                    if event.key() == Key::Enter {
                        goto_page()
                    }
                }
            },

            button {
                onclick: move |_| goto_page(),

                "submit",
            }
        }
    }
}
