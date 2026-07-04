use leptos::prelude::*;
//sqlite-wasm-rs = "0.5"
//
// sudo apt install clang
//
use sqlite_wasm_rs as ffi; //necessary as far as i can tell.
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

/*
 * relevant link, first thing i found:
 * https://github.com/leptos-rs/leptos/discussions/139 /
 *
 * this might be more relevant?
 * https://docs.rs/sqlite-wasm-rs/latest/sqlite_wasm_rs/
 */

#[component]
fn App() -> impl IntoView {
    let db = create_local_db_connection(); // returns *mut sqlite3

    on_cleanup(move || {
        if !db.is_null() {
            unsafe {
                ffi::sqlite3_close(db);
            }
        }
    });

    view! {
        <div class="container">
        </div>
    }
}

fn create_local_db_connection() -> *mut ffi::sqlite3 {
    let mut db = std::ptr::null_mut();
    let ret = unsafe {
        ffi::sqlite3_open_v2(
            c"mem.db".as_ptr().cast(),
            &mut db as *mut _,
            ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_CREATE,
            std::ptr::null(),
        )
    };
    assert_eq!(ffi::SQLITE_OK, ret);
    db
}
