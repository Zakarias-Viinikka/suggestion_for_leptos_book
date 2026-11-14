use gloo_timers::future::{TimeoutFuture, sleep};
use leptos::{prelude::*, task::spawn_local};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Parent);
}

#[component]
fn Parent() -> impl IntoView {
    let (all_done, all_done_set) = signal(false);
    view! {
        <Child on_finished=move |_| all_done_set.set(true)/>
        <div class="container">
            {move || if all_done.get() == true {
                view! {
                    "child is all finished doing their thing"
                }.into_any()
            } else {
                view! {
                    "waiting for child"
                }.into_any()
            }}
        </div>
    }
}

#[component]
fn Child(#[prop(into)] on_finished: Callback<()>) -> impl IntoView {
    spawn_local(async move {
        TimeoutFuture::new(2000).await;
        on_finished.run(());
    });
}
