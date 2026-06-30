use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*; //for the path!() macro

fn main() {
    console_error_panic_hook::set_once();
    //  trunk serve --open

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
      <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=DefaultPage/>
                <Route path=path!("/page1") view=Page1/>
                <Route path=path!("/page2") view=Page2/>
            </Routes>
      </Router>
    }
}

#[component]
fn DefaultPage() -> impl IntoView {
    view! {
        <br /><br /><br /><br /><br />
        <div class="container">
            "Default Page"
            <br/>
            <a href="page1">"Page 1"</a>
            <br/>
            <a href="page2">"Page 2"</a>
        </div>
    }
}

#[component]
fn Page1() -> impl IntoView {
    view! {
        <br /><br /><br /><br /><br />
        <div id="container">
            "Page 1"
        </div>
    }
}

#[component]
fn Page2() -> impl IntoView {
    view! {
        <br /><br /><br /><br /><br />
        <div id="container">
            "Page 2"
        </div>
    }
}
