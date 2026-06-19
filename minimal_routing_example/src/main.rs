use leptos::prelude::*;
use leptos_router::components::A; // for making <A> work
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path; // for the path!() macro

mod page1_and_page2;
use page1_and_page2::page1::Page1;
use page1_and_page2::page2;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=DefaultPage/>
                <Route path=path!("/page1") view=Page1/>
                <Route path=path!("/page2") view=page2::Page2/>
            </Routes>
        </main>
      </Router>
    }
}

#[component]
fn DefaultPage() -> impl IntoView {
    view! {
        <br/>   <br/>   <br/>
        <h2> "Default Page" </h2>
        <A href="/page1">"Page 1"</A>
        <br/>
        <A href="/page2">"Page 2"</A>
    }
}
