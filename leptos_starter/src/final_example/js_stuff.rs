use leptos::prelude::*;

#[component]
pub fn JsStuff() -> impl IntoView {
    view! {


        <script src="/js.js"></script>
        <script src="/Sortable.js" ></script>
    }
}
