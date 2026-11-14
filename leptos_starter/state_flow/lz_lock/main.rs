use leptos::prelude::*;

mod glob;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container">
            "this is what global variable holds: " {glob::ANYONE_CAN_ACCESS_ME.get()}
        </div>
    }
}
