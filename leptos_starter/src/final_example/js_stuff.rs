use leptos::prelude::*;

#[component]
pub fn JsStuff() -> impl IntoView {
    view! {
        <div class="container">
        </div>


        <script src="/js.js"></script>
        <script src="/Sortable.js" ></script>
    }
}
