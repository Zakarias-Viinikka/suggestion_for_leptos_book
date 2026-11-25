use gloo_timers::future::TimeoutFuture;
use leptos::{prelude::*, task::spawn_local};

#[component]
pub fn SignalCallback() -> impl IntoView {
    view! {
        <div class="container">
            <Parent/>
        </div>
    }
}

#[component]
fn Parent() -> impl IntoView {
    let cb = Callback::new(move |ctr: RwSignal<i32>| ctr.update(|ctr| *ctr += 1));

    view! {
        <Child callback=cb/>
    }
}

#[component]
fn Child(#[prop(into)] callback: Callback<RwSignal<i32>>) -> impl IntoView {
    let ctr = RwSignal::new(0);

    view! {
        <div>
            "ctr: " {move || ctr.get()}

            <button on:click=move |_| callback.run(ctr)>
                "run callback"
            </button>
        </div>
    }
}
