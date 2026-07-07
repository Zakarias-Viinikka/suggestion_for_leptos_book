//use crate::sqlite::read_from::{black_magic, db_table::Table};

use leptos::logging::log;
use leptos::prelude::*;

//cargo add gloo-timers
use gloo_timers::future::sleep;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let db_response = LocalResource::new(async move || test_async_fn().await);
    view! {
        <div class="container">
            <p>
            {move || {
                match db_response.get() {
                    None => view! { "db_response" <br/> "is" <br/> "empty" }.into_any(),
                    _ => view! {"db_response is not empty"}.into_any()
                }
            }}
            </p>
        </div>
    }
}

async fn test_async_fn() -> () {
    sleep(std::time::Duration::from_secs(2)).await;

    log!("2 seconds were supposed to pass");
    return ();
}
