// checkbox examples

use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (truth, truth_set) = signal(true);
    view! {
        //this checkbox updates a signal when clicked
        <input type="checkbox"
            prop:checked=truth
            on:change=move |ev| truth_set.set(event_target_checked(&ev))
        />
    }
}
