use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::boxes::{BoxController, BUTTONS};

#[component]
pub fn Page1Controls() -> impl IntoView {
    let c = expect_context::<BoxController>();
    let navigate = use_navigate();

    let on_next = move |_| {
        navigate("/page-2", Default::default());
    };

    view! {
        <nav class="nav-container">
            {BUTTONS
                .into_iter()
                .map(|(bx, by, label)| {
                    view! {
                        <button class="nav-btn" on:click=move |_| c.move_to(bx, by)>
                            {label}
                        </button>
                    }
                })
                .collect_view()}
        </nav>
        <div class="nav-container-next">
            <button class="nav-btn" on:click=on_next>
                "Next"
            </button>
        </div>
    }
}