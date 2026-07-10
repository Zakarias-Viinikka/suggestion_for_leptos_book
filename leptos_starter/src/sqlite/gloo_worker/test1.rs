use leptos::logging::log;
use leptos::prelude::*;

use gloo_worker::Spawnable;
//use gloo_worker::oneshot::oneshot;

use leptos_starter::sqlite::gloo_worker::shared::SharedThing;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    /*
    // this count is our synchronous, local state
    let (count, set_count) = signal(0);

    // tracks `count`, and reloads by calling `load_data`
    // whenever it changes
    let async_data = LocalResource::new(move || load_data(count.get()));
    */
    let (data, set_data) = signal("".to_string());
    LocalResource::new(move || async move {
        log!("test1");
        let bridge = SharedThing::spawner()
            .callback(move |response| {
                gloo_console::log!("gloo console log inside the callback");
                set_data.set(response);
            })
            .spawn("./worker.js");
        log!("test2");
        let bridge = Box::leak(Box::new(bridge));
        log!("test3");
        bridge.send("".to_string());
        log!("test4");
    });
    view! {
        <div class="container">
            "glue"
            <br/>
            {move || data.get()}
        </div>
    }
}
