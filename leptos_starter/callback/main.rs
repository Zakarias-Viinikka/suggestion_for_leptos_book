use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

mod fn_callback;
mod signal_callback;

use crate::fn_callback::FnCallback;
use crate::signal_callback::SignalCallback;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/signal_callback") view=SignalCallback/>
                    <Route path=path!("/fn_callback") view=FnCallback/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Callback Examples"</h1>
            <nav>
                <A href="/signal_callback">"Signal Callback"</A><br/>
                <A href="/fn_callback">"Function Callback"</A>
            </nav>
        </div>
    }
}
