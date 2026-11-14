use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                // add links here
            </nav>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=Home/>
                    // add routes here
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Leptos Starter"</h1>
        </div>
    }
}
