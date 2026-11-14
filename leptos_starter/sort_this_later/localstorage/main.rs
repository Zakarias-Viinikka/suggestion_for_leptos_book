// cargo add leptos_use
// cargo add codee --features json_serde

use codee::string::JsonSerdeCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (value, set_value, delete) = use_local_storage::<String, JsonSerdeCodec>("my-key");
    view! {
        <div class="container">
            <span>"This text input is stored in localstorage"</span><br />
            <input
                type="text"
                prop:value=move || value.get()
                on:input=move |ev| {
                    set_value.set(event_target_value(&ev));
                }
            />
            <button on:click=move |_| delete()>
                "Delete storage"
            </button>
        </div>
    }
}
