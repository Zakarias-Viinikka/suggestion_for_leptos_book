use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use leptos_starter::javascript_take_the_wheel;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (js_result, set_js_result) = signal("".to_string());
    javascript_take_the_wheel!("obey_me_rust", |js_value| {
        set_js_result.set(js_value.as_string().unwrap());
    });

    //leptos on load
    Effect::new(move |_| {
        make_worker_work();
    });
    view! {
        <div class="container">
            { move || {
                if js_result.get() == "".to_string() {
                    view! { "waiting for javascript to finish..." }.into_any()
                } else {
                    view!{ "javascript has done work. the result was: " <br/> {move || js_result.get()} }.into_any()
                }
            } }
        </div>
    }
}

#[wasm_bindgen]
extern "C" {
    fn make_worker_work();
}
