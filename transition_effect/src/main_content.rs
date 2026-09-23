use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use std::time::Duration;

#[component]
pub fn MainContent(
    opened: RwSignal<bool>,
    transition_done: RwSignal<bool>,
) -> impl IntoView {
    let navigate = use_navigate();
    let on_click = move |_| {
        opened.set(true);
        navigate("/page-b", Default::default());
        set_timeout(
            move || transition_done.set(true),
            Duration::from_secs(2),
        );
    };

    view! {
        <div class="layer main-content" class:opened=move || opened.get()>
            <button class="start-btn" on:click=on_click>
                "click"
            </button>
        </div>
    }
}