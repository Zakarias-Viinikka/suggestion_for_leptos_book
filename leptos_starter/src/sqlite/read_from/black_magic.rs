use js_sys::{Array, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, Url};
//for db
use leptos::logging::log;
use sqlite_wasm_rs as ffi; //necessary as far as i can tell.

use crate::sqlite::read_from::create_sql_statements::*;

pub fn create_local_db_connection() -> *mut ffi::sqlite3 {
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

pub fn test_db(db: *mut ffi::sqlite3) {
    let sql = c"CREATE TABLE test_data (value INTEGER); INSERT INTO test_data VALUES(42);";
    unsafe {
        let ret = ffi::sqlite3_exec(
            db,
            sql.as_ptr().cast(),
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert_eq!(ret, ffi::SQLITE_OK);
        log!("Table created and row inserted");
    }
}

pub fn export_db(db: *mut ffi::sqlite3) {
    unsafe {
        let mut size: ffi::sqlite3_int64 = 0;
        let data = ffi::sqlite3_serialize(db, c"main".as_ptr().cast(), &mut size, 0);

        if data.is_null() {
            log!("Failed to serialize database");
            return;
        }

        // Create a slice from the raw data pointer [citation:4]
        let bytes = std::slice::from_raw_parts(data as *const u8, size as usize);

        // Create a Uint8Array from the byte slice
        let uint8_array = js_sys::Uint8Array::view(bytes);

        // Create the Blob from the Array [citation:6][citation:10]
        let blob = Blob::new_with_u8_array_sequence(&js_sys::Array::of1(&uint8_array)).unwrap();

        // Create object URL and trigger download [citation:6]
        let url = Url::create_object_url_with_blob(&blob).unwrap();
        let a: web_sys::HtmlElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("a")
            .unwrap()
            .unchecked_into();

        a.set_attribute("href", &url).unwrap();
        a.set_attribute("download", "database.sqlite").unwrap();
        a.click();
        /*
         */
        //memory freeing stuff or whatever
        ffi::sqlite3_free(data as *mut std::ffi::c_void);
    }
}
