use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

/*
 *
 * effect only runs when 1 signal updates but not the other while depending on both
 */

#[component]
fn App() -> impl IntoView {
    let (number_tied_to_button, number_tied_to_button_set) = signal(0);
    let (number_tied_to_button2, number_tied_to_button_set2) = signal(0);
    let (tied_to_nobody, tied_to_nobody_set) = signal(0);

    Effect::new(move || {
        tied_to_nobody_set
            .set(number_tied_to_button.get() + number_tied_to_button2.get_untracked());
    });
    view! {
        <div class="container">
            <h4>"Increase Number 1: " {move || number_tied_to_button.get()}</h4>
            <button on:click=move |_| number_tied_to_button_set.update(|v| *v += 1)>"Click"</button>

            <br/>
            <br/>


            <h4>"Increase Number 2: " {move || number_tied_to_button2.get()}</h4>
            <button on:click=move |_| number_tied_to_button_set2.update(|v| *v += 1)>"Click"</button>

            <br/>
            <br/>
            <hr/>
            <h4> "this update happens based on the signal changing" </h4>
            <h4> "tied to nobody: " {move || tied_to_nobody.get()}</h4>

        </div>
    }
}
