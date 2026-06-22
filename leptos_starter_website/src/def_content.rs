use leptos::prelude::*;

#[component]
pub fn MainContent() -> impl IntoView {
    view! {
        <Title />
        <ContentBody />
    }
}

#[component]
fn ContentBody() -> impl IntoView {
    view! {
        <div class="content-body">
        <br /><br /><br /><br />
            "test"
        </div>
    }
}

#[component]
fn Title() -> impl IntoView {
    view! {
        <h1 class="title">"Title"</h1>
    }
}
