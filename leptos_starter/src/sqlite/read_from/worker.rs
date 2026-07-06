//use leptos_starter::sqlite::read_from::{black_magic, db_table::Table};
use leptos_workers::worker;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum DbResponse {
    Success,
    Error { message: String },
}

//cargo add leptos_workers
// cargo add serde --features derive
#[worker(DoDbStuff)]
pub async fn worker(/*table: Table*/ abc: bool) -> DbResponse {
    // Step 1: Connect
    /*let conn = match black_magic::create_local_db_connection("default-db").await {
        Ok(c) => c,
        Err(e) => {
            return DbResponse::Error {
                message: e.to_string(),
            };
        }
    };*/
    /*
        // Step 2: Create table
        if let Err(e) = black_magic::create_table(conn, &table) {
            return DbResponse::Error {
                message: e.to_string(),
            };
        }

        // Step 3: prepare data then
        let values = vec![
            "your text content here".to_string(),
            "{\"key\": \"value\"}".to_string(),
        ];
        //Step 3.5 Insert
    */
    /*if let Err(e) = black_magic::insert_into_table(conn, &table, values) {
        return DbResponse::Error {
            message: e.to_string(),
        };
    }*/

    // All steps succeeded
    DbResponse::Success
}

fn main() {
    // The leptos_workers macro handles the rest.
    // This main is required for Trunk to build it as a separate binary.
}
