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
        <Child all_done=all_done_set/>
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
fn Child(all_done: WriteSignal<bool>) -> impl IntoView {
    spawn_local(async move {
        TimeoutFuture::new(2000).await;
        all_done.set(true)
    });
    view! {}
}
