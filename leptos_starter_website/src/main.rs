use leptos::prelude::*;
//use leptos_router::components::A;
//use leptos_router::components::{Route, Router, Routes};
//use leptos_router::*;

mod def_content;
use def_content::*;

mod background;
use background::*;

fn main() {
    console_error_panic_hook::set_once();
    //  trunk serve --open

    mount_to_body(App);
}

fn App() -> impl IntoView {
    view! {
        <MainContent />
        <Background />
    }
}
