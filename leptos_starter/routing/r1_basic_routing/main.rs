use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    //  trunk serve --open

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Component1/>
        <Component2/>
    }
}

#[component]
fn Component1() -> impl IntoView {
    view! {
        <div class="container">
            "component 1"
        </div>
    }
}

#[component]
fn Component2() -> impl IntoView {
    view! {
        <div class="container">
            "component 2"
        </div>
    }
}
