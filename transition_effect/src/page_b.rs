use leptos::prelude::*;

#[component]
pub fn PageB() -> impl IntoView {
    view! {
        <div class="page-b">
            <h1>"Page B"</h1>
            <p>"This page was loaded into the slot."</p>
        </div>
    }
}
