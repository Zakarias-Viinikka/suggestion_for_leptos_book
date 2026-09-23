use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (number, number_set) = signal(0);

    //runs once on load
    Effect::new(move |_| {
        number_set.set(6);
    });
    view! {
        <div class="container">
            <p>{move || number.get()}</p>
        </div>
    }
}
