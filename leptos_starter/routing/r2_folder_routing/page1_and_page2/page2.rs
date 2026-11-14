use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Page2() -> impl IntoView {
    view! {
        <br/>   <br/>   <br/>
        <h2> "Page 2" </h2>
        <A href="/">"Link back to Default"</A>
        <br/>
        <A href="/page1">"Link to Page 1"</A>
    }
}
