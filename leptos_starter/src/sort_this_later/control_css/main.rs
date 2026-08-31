use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (size, set_size) = signal(50);

    view! {
        <div
            style:height="160px"
            class="container"
            >
            <span>"Change size of the box"</span>

            <input
                type="range"
                min="0"
                max="100"
                prop:value=size
                on:input:target=move |ev| {
                    set_size.set(ev.target().value().parse().unwrap());
                }
            />

            <div
                style:border="1px solid black"
                style:width=move || format!("{}px", size.get())
                style:height=move || format!("{}px", size.get())
            >
            </div>
        </div>
    }
}
