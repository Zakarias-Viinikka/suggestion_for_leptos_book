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

    let db = black_magic::create_local_db_connection("default-db"); //  returns Result<*mut sqlite3, Error>
    match db {
        Ok(db) => {
            match black_magic::create_table(db, &table) {
                Ok(_) => {
                    let values = vec![
                        "your text content here".to_string(),
                        "{\"key\": \"value\"}".to_string(), // JSON for metadata
                    ];
                    match black_magic::insert_into_table(db, &table, values) {
                        Ok(_) => {
                            log!("everything went well with db");
                            //black_magic::export_db(db);
                        }
                        Err(e) => {
                            log!("Error inserting into table: {}", e)
                        }
                    }
                }
                Err(e) => {
                    log!("Error creating table: {}", e)
                }
            }
        }
        Err(e) => {
            log!("Error creating database: {}", e)
        }
    }
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
        </div>
    }
}
