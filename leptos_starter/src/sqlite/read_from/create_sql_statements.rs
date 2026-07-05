pub enum ColumnType {
    Integer,
    Text,
    Real, // for floating-point numbers
    Blob, // for binary data
}

impl ColumnType {
    fn as_str(&self) -> &'static str {
        match self {
            ColumnType::Integer => "INTEGER",
            ColumnType::Text => "TEXT",
            ColumnType::Real => "REAL",
            ColumnType::Blob => "BLOB",
        }
    }
}

pub struct Column {
    pub column_name: String,     // now public
    pub column_type: ColumnType, // now public
}

pub struct Table {
    pub table_name: String,   // now public
    pub columns: Vec<Column>, // now public
}

pub fn prepare_sql_for_create_table(table: Table) -> String {
    //CREATE TABLE IF NOT EXISTS test_data (value INTEGER);
    let cols: Vec<String> = table
        .columns
        .iter()
        .map(|c| format!("{} {}", c.column_name, c.column_type.as_str()))
        .collect();

    format!(
        "CREATE TABLE IF NOT EXISTS {} ({});",
        table.table_name,
        cols.join(", ")
    )
}

/*
enum HappySql { SanitizedSqlInput(String)) }

together with a method i would have like fn sanitize_userinput_to_sql(input: String) -> SanitizedSqlInput(String)) {}
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_sql_for_create_table() {
        let table = Table {
            table_name: "users".to_string(),
            columns: vec![
                Column {
                    column_name: "id".to_string(),
                    column_type: ColumnType::Integer,
                },
                Column {
                    column_name: "name".to_string(),
                    column_type: ColumnType::Text,
                },
                Column {
                    column_name: "age".to_string(),
                    column_type: ColumnType::Real,
                },
            ],
        };

        let sql = prepare_sql_for_create_table(table);
        let expected = "CREATE TABLE IF NOT EXISTS users (id INTEGER, name TEXT, age REAL);";
        assert_eq!(sql, expected);
    }
}
