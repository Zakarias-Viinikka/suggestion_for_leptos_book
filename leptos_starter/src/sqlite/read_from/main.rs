use leptos::logging::log;
use leptos::prelude::*;
//use sqlite-wasm-rs = "0.5"
//
// sudo apt install clang
//
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

use leptos_starter::sqlite::read_from::black_magic;
use leptos_starter::sqlite::read_from::create_sql_statements::*;

use sqlite_wasm_rs::{
    self as ffi,
    sahpool_vfs::{OpfsSAHPoolCfg, install as install_opfs_sahpool},
};
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
    //sahpool doesn't support multiple connections which means that if the website is opened in another tab the connection fails.
    // work around is to either use 2 storages. default to sahpool and otherwise use indexeddb
    // other alternative is to more manually drop the connection if it hasn't been used in a while and try to reconnect when you need it
    // // tracks `count`, and reloads by calling `load_data`
    // whenever it changes

    let _db_loader = LocalResource::new(move || {
        let table = table.clone(); // clone here, keep original in closure
        async move {
            // use table (the cloned one) - it's moved into the future
            match black_magic::create_local_db_connection("default-db").await {
                Ok(conn) => {
                    if black_magic::create_table(conn, &table).is_ok() {
                        let values = vec![
                            "your text content here".to_string(),
                            "{\"key\": \"value\"}".to_string(),
                        ];
                        if let Err(e) = black_magic::insert_into_table(conn, &table, values) {
                            log!("Error inserting: {}", e);
                        } else {
                            log!("everything went well");
                        }
                    }
                }
                Err(e) => log!("Failed to open DB: {}", e),
            }
        }
    });
    //black_magic::test_db(db);
    //black_magic::export_db(db);

    /*on_cleanup(move || {
        if !db.is_null() {
            unsafe {
                ffi::sqlite3_close(db);
            }
        }
    });*/
    view! {
        <div class="container">
        "hello"
        </div>
    }
}
