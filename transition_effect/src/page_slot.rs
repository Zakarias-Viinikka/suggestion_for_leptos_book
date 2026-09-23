use leptos::prelude::*;
use leptos_router::components::{Route, Routes};
use leptos_router::path;

use crate::page_b::PageB;

#[component]
pub fn PageSlot() -> impl IntoView {
    view! {
        <div class="layer page-slot">
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=|| () />
                <Route path=path!("/page-b") view=PageB />
            </Routes>
        </div>
    }
}