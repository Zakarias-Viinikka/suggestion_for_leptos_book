use leptos::prelude::*;
use crate::execute_download::execute_download;

#[component]
pub fn AppMain() -> impl IntoView {
    let do_download = move || {
        let things = get_things_to_download();
        let things_as_json: Vec<String> = things.into_iter().map(|t| thing_to_json(t.0)).collect();
        execute_download(things_as_json, "json");
    };
    view! {
        <div class="container">
            <button on:click=move |_| {
                do_download();
            }>
            "download"
            </button>
        </div>
    }
}

fn thing_to_json(thing_to_convert: String) -> String {
    let t = thing_to_convert;

    serde_json::json!({
        "content": t,
        "metadata": "fake_metadata",
        "number": 0,
    })
    .to_string()
}

fn get_things_to_download() -> Vec<ThingsToDownload> {
    let page1 = "Page one.

This is the first page.";
    let page2 = "Page two.

This is the second page.";
    let page3 = "Page three.

This is the third page.";
    vec![
        ThingsToDownload(page1.to_string()),
        ThingsToDownload(page2.to_string()),
        ThingsToDownload(page3.to_string()),
    ]
}

struct ThingsToDownload(String);
