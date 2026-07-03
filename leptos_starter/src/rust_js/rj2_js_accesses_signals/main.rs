use anyhow::{Result, anyhow};
use leptos::logging::log;
use leptos::prelude::*; // easier error messages
//use leptos_meta::*; //<Script>
use wasm_bindgen::{JsCast, JsValue}; // Needed for .dyn_into()

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (signal, signal_set) = signal(0);

    // If you have questions. I don't have answers, i just found this: https://github.com/leptos-rs/leptos/discussions/2423#discussioncomment-8767056
    let handle = window_event_listener_untyped("update_rust_signal_from_javascript", move |ev| {
        // 3. Cast the generic Event to a CustomEvent
        if let Ok(custom_ev) = ev.dyn_into::<web_sys::CustomEvent>() {
            let number = js_value_to_i32(custom_ev.detail());
            match number {
                Ok(num) => signal_set.set(num),
                Err(e) => {
                    log!("{}", e);
                }
            }
        }
    });

    // js to rust gets messy. Manual memory managemet or something right here. live with it.
    // without it you create a handle every time you load the component.
    on_cleanup(move || {
        handle.remove();
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
