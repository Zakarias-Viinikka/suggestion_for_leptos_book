use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Page1() -> impl IntoView {
    view! {
        <br/>   <br/>   <br/>
        <h2> "Page 1" </h2>
        <A href="/">"Link back to Default"</A>
        <br/>
        <A href="/page2">"Link to Page 2"</A>
    }
}
