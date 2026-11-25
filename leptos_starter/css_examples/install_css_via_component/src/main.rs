use leptos::prelude::*;
use leptos_meta::*; //<Stylesheet/>

use leptos_router::components::{Route, Router, Routes};
use leptos_router::path; // for the path!() macro

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

/*
   cargo add stylance
*/

#[component]
fn App() -> impl IntoView {
    provide_meta_context(); //this seems to be important? idk. removing it didn't break stuff in this project atleast.
    view! {
        <Router>
          <nav>
            /* ... */
          </nav>
          <main>
              <Routes fallback=|| "Not found.">
                  <Route path=path!("/") view=StyledPages/>
                  <Route path=path!("/pageWithoutStyling") view=PageWithoutStyling/>        //<- both work
              </Routes>
          </main>
        </Router>
    }
}

#[component]
pub fn StyledPages() -> impl IntoView {
    view! {
        <Page1 />
        <br/>
        <Page2 />

        <a href="/pageWithoutStyling">"Page without styling"</a>
    }
}

#[component]
pub fn Page1() -> impl IntoView {
    view! {
        <Stylesheet href="/css/red_border.css"/>
        <h1>"Page 1"</h1>
    }
}

#[component]
pub fn Page2() -> impl IntoView {
    view! {
        <Stylesheet href="/css/padding.css"/>
        <h1>"Page 2"</h1>
    }
}

#[component]
pub fn PageWithoutStyling() -> impl IntoView {
    view! {
        <h1>"Page Without Styling"</h1>
    }
}
