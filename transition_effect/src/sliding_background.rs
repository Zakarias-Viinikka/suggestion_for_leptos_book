use leptos::prelude::*;

#[component]
pub fn SlidingBackground(opened: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="layer slider" class:opened=move || opened.get()>
            <div class="panel left"></div>
            <div class="panel right"></div>
        </div>
    }
}
