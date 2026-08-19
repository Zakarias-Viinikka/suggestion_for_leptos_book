use leptos::prelude::*;
use rand::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let random_number_1 = RwSignal::new(one_or_zero());
    let random_number_2 = RwSignal::new(one_or_zero());
    view! {
        <div class="container">
            <Show
                when=move || random_number_1.get() == 0
                fallback=|| view! {"first number isn't one so i'm throwing a fit without even looking at the second one"}
            >
                <Show
                    when=move || random_number_2.get() == 0
                    fallback=|| view! {"first number was a one but second was a 0. I'm upset but i'll accept"}
                >
                "both numbers are 1 and life is great. don't reload the page or my fun will be ruined"
                </Show>
            </Show>
        </div>
    }
}

fn one_or_zero() -> u8 {
    rand::rng().random_range(0..2)
}
