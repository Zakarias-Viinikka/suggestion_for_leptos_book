//use js_sys::{Array, Uint8Array};
use wasm_bindgen::{JsCast /*JsValue*/};
use web_sys::{Blob, Url};
//for db
use leptos::logging::log;
use sqlite_wasm_rs as ffi; //necessary as far as i can tell.

use crate::sqlite::read_from::create_sql_statements::*;

use anyhow::{Result, anyhow, bail};
use std::ffi::CString; //let sql_cstr = CString::new(sql).map_err(|e| anyhow!("CString conversion failed: {}", e))?;

pub fn create_local_db_connection(filename: &str) -> Result<*mut ffi::sqlite3> {
    let filename_cstr = CString::new(filename)?; // converts &str to CString, errors if contains nul byte
    let mut db = std::ptr::null_mut();
    let ret = unsafe {
        ffi::sqlite3_open_v2(
            filename_cstr.as_ptr().cast(),
            &mut db as *mut _,
            ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_CREATE,
            std::ptr::null(),
        )
    };
    if ret != ffi::SQLITE_OK {
        bail!("Failed to open database: {}", ffi::code_to_str(ret));
    }
    Ok(db)
}

/*
let sql = format!("CREATE TABLE {} ({});", table_name, column_defs);
let c_sql = std::ffi::CString::new(sql).unwrap();
// then pass c_sql.as_ptr().cast() to sqlite3_exec
*/
pub fn create_table(db: *mut ffi::sqlite3, table: &Table) -> Result<()> {
    let sql = generate_create_table_sql(table);
    let sql_cstr = CString::new(sql).map_err(|e| anyhow!("CString conversion failed: {}", e))?;
    unsafe {
        let ret = ffi::sqlite3_exec(
            db,
            sql_cstr.as_ptr().cast(),
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if ret != ffi::SQLITE_OK {
            bail!("Failed to create table: {}", ffi::code_to_str(ret));
        }
        log!("Table created");
        Ok(())
    }
}

pub fn insert_into_table(db: *mut ffi::sqlite3, table: &Table, values: Vec<String>) -> Result<()> {
    let sql = generate_insert_sql(table, values);
    let sql_cstr = CString::new(sql)?;
    unsafe {
        let ret = ffi::sqlite3_exec(
            db,
            sql_cstr.as_ptr(),
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if ret != ffi::SQLITE_OK {
            bail!("insert failed: {}", ffi::code_to_str(ret));
        }
    }
    Ok(())
}

pub fn export_db(db: *mut ffi::sqlite3) -> Result<()> {
    unsafe {
        let mut size: ffi::sqlite3_int64 = 0;
        let data = ffi::sqlite3_serialize(db, c"main".as_ptr().cast(), &mut size, 0);

        if data.is_null() {
            log!("Failed to serialize database");
            bail!("Failed to serialize database");
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
        Ok(())
    }
}

//
// //
//
// test in tests/web.rs
// had to do weird shit to get it to work.
// wasm-pack test --headless --firefox
// wasm-pack test --headless --firefox
// wasm-pack test --headless --firefox
//
// //
//
