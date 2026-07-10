use serde::{Deserialize, Serialize};

use leptos::logging::log;
use leptos::prelude::*;

//cargo add gloo-timers
use gloo_timers::future::sleep;

//for the actual worker stuff
// cargo add gloo-worker --features futures
use gloo_worker::Spawnable;
use gloo_worker::oneshot::oneshot;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // https://docs.rs/gloo-worker/latest/gloo_worker/#worker-trait
    /*let response = LocalResource::new(move || async move {
        let mut caller = do_work_on_different_thread::spawner().spawn("./worker.js");
        caller.run("input".to_string()).await
    });*/

    let response = LocalResource::new(move || async move {
        let mut caller = do_work_on_different_thread::spawner().spawn("./worker.js");
        caller.run("input".to_string()).await
    });

    /*let work_caller = Multiplier::spawner()
        .callback(move |_| {
            do_work_on_different_thread
        })
        .spawn("./worker.js");
    let work_caller = Box::leak(Box::new(work_caller));

    work_caller.send();*/

    view! {
        <div class="container">
            /*<p>
            {move || {
                match db_response.get() {
                    None => view! { "db_response is empty" },
                    _ => view! {"de_response is not empty"}
                }
            }}
            </p>*/
        </div>
    }
}

#[oneshot]
async fn do_work_on_different_thread(my_request: String) -> MyResponse {
    sleep(std::time::Duration::from_secs(2)).await;
    MyResponse::Success
}

#[derive(Clone, Serialize, Deserialize)]
pub enum MyResponse {
    Success,
    Error { message: String },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MyRequest;
