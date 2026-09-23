use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::path;

mod boxes;
mod page1_controls;
mod page2;

use boxes::{BoxController, Theme7Boxes};
use page1_controls::Page1Controls;
use page2::Page2;

fn main() {
    console_error_panic_hook::set_once();
    // trunk serve --open

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Inner />
        </Router>
    }
}

#[component]
fn Inner() -> impl IntoView {
    let location = use_location();

    let initial_path = location.pathname.get_untracked();
    let initial_spread = if initial_path == "/page-2" { 150.0 } else { 0.0 };
    let controller = BoxController::new(50.0, 50.0, initial_spread);
    provide_context(controller);

    Effect::new(move |_| {
        let path = location.pathname.get();
        let target = if path == "/page-2" { 150.0 } else { 0.0 };
        controller.spread.set(target);
    });

    view! {
        <main class="page-slot">
            <Routes fallback=|| "Not found">
                <Route path=path!("/") view=|| () />
                <Route path=path!("/page-2") view=Page2 />
            </Routes>
        </main>
        <Theme7Boxes />
        {move || {
            (location.pathname.get() == "/").then(|| view! { <Page1Controls /> })
        }}
    }
}