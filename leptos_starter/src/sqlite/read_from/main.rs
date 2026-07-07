use leptos::logging::log;
use leptos::prelude::*;
//use sqlite-wasm-rs = "0.5"
//
// sudo apt install clang
// i don't remember why i had to install clang? but it fixed something.
//
//
/*use sqlite_wasm_rs::{
    self as ffi,
    sahpool_vfs::{OpfsSAHPoolCfg, install as install_opfs_sahpool},
};*/

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

use leptos_starter::sqlite::read_from::db_table::*;
use leptos_starter::sqlite::read_from::worker::{DbResponse, worker};
/*
 * relevant link, first thing i found:
 * https://github.com/leptos-rs/leptos/discussions/139 /
 *
 * this might be more relevant?
 * https://docs.rs/sqlite-wasm-rs/latest/sqlite_wasm_rs/
 */

#[component]
fn App() -> impl IntoView {
    let table = Table {
        table_name: "content".to_string(),
        columns: vec![
            Column {
                column_name: "textblock".to_string(),
                column_type: ColumnType::Text,
            },
            Column {
                column_name: "metadata".to_string(),
                column_type: ColumnType::Text,
            },
        ],
    };

    /*let db_response = LocalResource::new(move || {
        let table_clone = table.clone(); // clone here, outside async block
        async move {
            worker_test(/*table_clone*/ true).await // use the clone, not the original
        }
    });*/
    //sahpool doesn't support multiple connections which means that if the website is opened in another tab the connection fails.
    // work around is to either use 2 storages. default to sahpool and otherwise use indexeddb
    // other alternative is to more manually drop the connection if it hasn't been used in a while and try to reconnect when you need it
    // // tracks `count`, and reloads by calling `load_data`
    // whenever it changes

    /*on_cleanup(move || {
        if !db.is_null() {
            unsafe {
                ffi::sqlite3_close(db);
            }
        }
    });*/
    view! {
        <div class="container">

        <p>/*
            { move || match db_response.get() {
                Some(Ok(DbResponse::Success)) => "DB ready".to_string(),
                Some(Ok(DbResponse::Error { message })) => format!("DB error: {}", message),
                Some(Err(e)) => format!("Worker error: {}", e),
                None => "Loading...".to_string(),
            } }*/
            /*<p>
            {move || {
                match db_response.get() {
                    None => view! { "db_response is empty" },
                    _ => view! {"de_response is not empty"}
                }
            }}
            </p>*/
        </p>

        </div>
    }
}

/*//cargo add leptos_workers
// cargo add serde --features derive
#[worker(DoDbStuff)]
pub async fn worker_test(/*table: Table*/ abc: bool) -> DbResponse {
    DbResponse::Success
}*/
