use leptos::prelude::*;
use leptos_starter::state_flow::once_lock::glob;

fn main() {
    console_error_panic_hook::set_once();
    glob::ANYONE_CAN_ACCESS_ME
        .set(RwSignal::new("hello from OnceLock".into()))
        .ok();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container">
            "this is what the once lock holds: " {move || glob::ANYONE_CAN_ACCESS_ME.get()}
        </div>
    }
}