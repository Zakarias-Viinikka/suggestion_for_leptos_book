use leptos::prelude::*;

#[component]
pub fn FnCallback() -> impl IntoView {
    view! {
        <div class="container">
            <Child callback=my_function/>
        </div>
    }
}

fn my_function() {
    // placeholder
}

#[component]
fn Child(callback: fn()) -> impl IntoView {
    let _ = callback;
    view! {
        <div>
        </div>
    }
}