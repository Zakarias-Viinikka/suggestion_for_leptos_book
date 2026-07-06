//use crate::sqlite::read_from::{black_magic, db_table::Table};
use leptos_workers::worker;
use serde::{Deserialize, Serialize};

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
    //let db_response = LocalResource::new(async move || worker_test(MyRequest).await);
    let tmp = LocalResource::new(async move || simple_ping("".to_string()).await);
    view! {
        <div class="container">
            /*<p>
            {move || {
                match db_response.get() {
                    None => view! { "db_response is empty" },
                    _ => view! {"de_response is not empty"}
                }
            }}
            </p>*/""
        </div>
    }
}

//cargo add leptos_workers
// cargo add serde --features derive
#[worker(MyFutureWorker)]
async fn worker_test(my_request: MyRequest) -> MyResponse {
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

#[worker]
async fn simple_ping(a: String) -> String {
    "pong".to_string()
}
