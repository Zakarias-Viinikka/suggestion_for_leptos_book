// tests/web.rs
// wasm-pack test --headless --firefox
// wasm-pack test --headless --firefox
// wasm-pack test --headless --firefox
// wasm-pack test --headless --firefox
//
// im gonna try to remember how i got it to work if future me has to try and do same thing again
// *toml needed wasm-bindgen-test = "0.3.76"
// *#![cfg(target_arch = "wasm32")] at the top of the test
// *pub on everything
// *#[wasm_bindgen_test] instead of [test]
// *moved the tests to tests/web.rs
// *changed crate:: to root_name::
// *cargo install wasm-pack
//
#![cfg(target_arch = "wasm32")]

use anyhow::{Result, anyhow, bail}; // ← add `bail` here
use leptos_starter::sqlite::read_from::black_magic::*;
use leptos_starter::sqlite::read_from::create_sql_statements::*;
use sqlite_wasm_rs as ffi;
use std::ffi::CString;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

pub fn open_test_db() -> Result<*mut ffi::sqlite3> {
    create_local_db_connection("test.db")
}

pub fn delete_test_table(db: *mut ffi::sqlite3, table_name: &str) -> Result<()> {
    let sql = format!("DROP TABLE IF EXISTS {};", table_name);
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
            bail!("Failed to drop table: {}", ffi::code_to_str(ret));
        }
    }
    Ok(())
}

#[wasm_bindgen_test]
pub fn test_create_table_works() -> Result<()> {
    let db = open_test_db()?; // uses test file, not production

    let table = Table {
        table_name: "test_content".to_string(),
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

    create_table(db, &table)?;
    Ok(())
    // database file "test_data.db" remains after test – you can drop the table or delete the file here if you want
}

#[wasm_bindgen_test]
pub fn test_create_table() -> Result<()> {
    let db = open_test_db()?;
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
    create_table(db, &table)?;
    delete_test_table(db, &table.table_name)?;
    Ok(())
}

#[wasm_bindgen_test]
pub fn test_insert_into_table() -> Result<()> {
    let db = open_test_db()?;
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
    create_table(db, &table)?;
    let values = vec!["Hello, world!".to_string(), r#"{"foo": "bar"}"#.to_string()];
    insert_into_table(db, &table, values)?;
    delete_test_table(db, &table.table_name)?;
    Ok(())
}
