use leptos::prelude::*;
use wasm_bindgen::prelude::*; //for being able to link to the js file

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (signal_get, signal_set) = signal("".to_string());

    view! {
        <button on:click=move |_| {call_js_func(signal_set)}> <h2>"I'm a button"</h2> </button>
        <br/><br/>
        "Javscript said: " {signal_get} //
        //I had to add thsi to index.html other alternative were way more annoying: <link data-trunk rel="inline" type="js" href="./js/calling_rust_from_js.js" />
    }
}

fn call_js_func(signal: WriteSignal<String>) {
    signal.set(js_func())
}

#[wasm_bindgen]
extern "C" {
    fn js_func() -> String;
}
