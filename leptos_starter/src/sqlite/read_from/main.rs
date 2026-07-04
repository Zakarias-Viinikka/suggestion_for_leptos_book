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

//for db
use sqlite_wasm_rs as ffi; //necessary as far as i can tell.
//for exporting the db
use js_sys::{Array, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, Url};

use leptos_starter::sqlite::read_from::black_magic;
//for exporting the db

/*
 * relevant link, first thing i found:
 * https://github.com/leptos-rs/leptos/discussions/139 /
 *
 * this might be more relevant?
 * https://docs.rs/sqlite-wasm-rs/latest/sqlite_wasm_rs/
 */

#[component]
fn App() -> impl IntoView {
    let db = black_magic::create_local_db_connection(); // returns *mut sqlite3
    black_magic::test_db(db);
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
