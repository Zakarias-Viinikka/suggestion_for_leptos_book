use anyhow::{Result, anyhow};
use leptos::logging::log;
use leptos::prelude::*; // easier error messages
use leptos_starter::javascript_take_the_wheel; //macro
use wasm_bindgen::JsValue; // Needed for js_value_to_i32

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (signal, signal_set) = signal(0);

    javascript_take_the_wheel!("update_rust_signal_from_javascript", |js_value| {
        match js_value_to_i32(js_value) {
            Ok(num) => signal_set.set(num), //set signal if everything looks good
            Err(e) => log!("{}", e),        //console.log error
        }
    });

    view! {
        <div class="container">
        "num: " {move || signal}
        </div>
        <Javascript/>
    }
}

#[component]
fn Javascript() -> impl IntoView {
    view! {
        //needed to cast x to a string 'String(x)' send x away to rust since i wanted to convert it to a String from a JsValue
        <script>"
            let x = 1;
            setInterval(() => {
                x++;
                const event = new CustomEvent('update_rust_signal_from_javascript', {
                    detail: String(x)
                });
                window.dispatchEvent(event);
            }, 1000);
        "</script>
    }
}

fn js_value_to_i32(value: JsValue) -> Result<i32> {
    let num = value
        .as_string() // I wanted to use a string because it's easier to convert to i32 with error messages than f64
        .ok_or(anyhow!("couldn't get a string from js"))?
        .parse::<i32>()
        .map_err(|e| anyhow!("error: {}", e))?;
    Ok(num)
}
