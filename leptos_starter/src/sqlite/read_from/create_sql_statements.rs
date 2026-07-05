//to run test:
// cargo test --lib --target x86_64-unknown-linux-gnu
//

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

pub fn generate_create_table_sql(table: Table) -> String {
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

pub fn generate_insert_sql(table: Table, values: Vec<String>) -> String {
    let quoted_values: Vec<String> = values
        .iter()
        .map(|v| format!("'{}'", sanitize(v.as_ref())))
        .collect();
    format!(
        "INSERT INTO {} ({}) VALUES ({});",
        table.table_name,
        table
            .columns
            .iter()
            .map(|c| c.column_name.clone())
            .collect::<Vec<_>>()
            .join(", "),
        quoted_values.join(", ")
    )
}
pub fn generate_swap_two_values_sql(
    id1: usize,
    id2: usize,
    table_name: String,
    column_name: String,
) -> String {
    format!(
        "
    UPDATE {table}
    SET {column} = CASE
        WHEN id = {id1} THEN (SELECT {column} FROM {table} WHERE id = {id2})
        WHEN id = {id2} THEN (SELECT {column} FROM {table} WHERE id = {id1})
    END
    WHERE id IN ({id1}, {id2});
    ",
        table = table_name,
        column = column_name,
        id1 = id1,
        id2 = id2
    )
}

pub fn generate_delete_sql(id: usize, table_name: String) -> String {
    format!(
        "DELETE FROM {table} WHERE id = {id};",
        table = table_name,
        id = id
    )
}

//UPDATE users SET name = 'Bob', age = 30 WHERE id = 3;
pub fn generate_update_sql<I, K, V>(
    id: usize,
    table_name: &str,
    columns_and_new_values: I,
) -> String
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>, // column name can be &str or String
    V: AsRef<str>, // value can be &str or String
{
    let col_and_val = columns_and_new_values
        .into_iter()
        .map(|(col, val)| {
            let sanitized_val = sanitize(val.as_ref());
            format!("{} = '{}'", col.as_ref(), sanitized_val)
        })
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "UPDATE {table} SET {col_and_val} WHERE id = {id};",
        table = table_name,
        col_and_val = col_and_val,
        id = id
    )
}

fn sanitize(input: &str) -> String {
    input.replace("'", "''")
}

/*
enum HappySql { SanitizedSqlInput(String)) }

together with a method i would have like fn sanitize_userinput_to_sql(input: String) -> SanitizedSqlInput(String)) {}
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_create_table_sql() {
        let table = Table {
            table_name: "employees".to_string(),
            columns: vec![
                Column {
                    column_name: "emp_id".to_string(),
                    column_type: ColumnType::Integer,
                },
                Column {
                    column_name: "first_name".to_string(),
                    column_type: ColumnType::Text,
                },
                Column {
                    column_name: "salary".to_string(),
                    column_type: ColumnType::Real,
                },
            ],
        };

        let sql = generate_create_table_sql(table);
        let expected =
            "CREATE TABLE IF NOT EXISTS employees (emp_id INTEGER, first_name TEXT, salary REAL);";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_generate_insert_sql() {
        let table = Table {
            table_name: "products".to_string(),
            columns: vec![
                Column {
                    column_name: "product_id".to_string(),
                    column_type: ColumnType::Integer,
                },
                Column {
                    column_name: "product_name".to_string(),
                    column_type: ColumnType::Text,
                },
            ],
        };
        let values = vec!["100".to_string(), "Laptop".to_string()];
        let sql = generate_insert_sql(table, values);
        let expected = "INSERT INTO products (product_id, product_name) VALUES ('100', 'Laptop');";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_generate_swap_two_values_sql() {
        let sql = generate_swap_two_values_sql(5, 10, "inventory".to_string(), "stock".to_string());

        assert!(sql.contains("UPDATE inventory"));
        assert!(sql.contains("SET stock = CASE"));
        assert!(sql.contains("WHEN id = 5 THEN (SELECT stock FROM inventory WHERE id = 10)"));
        assert!(sql.contains("WHEN id = 10 THEN (SELECT stock FROM inventory WHERE id = 5)"));
        assert!(sql.contains("WHERE id IN (5, 10)"));
    }

    #[test]
    fn test_generate_delete_sql() {
        let sql = generate_delete_sql(42, "orders".to_string());
        let expected = "DELETE FROM orders WHERE id = 42;";
        assert_eq!(sql, expected);
    }

    //pub fn generate_update_sql<I, K, V>(id: usize, table_name: &str, columns_and_new_values: I) -> String
    #[test]
    fn test_generate_update_sql_single() {
        let sql = generate_update_sql(5, "employees", vec![("name", "Alice")]);
        let expected = "UPDATE employees SET name = 'Alice' WHERE id = 5;";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_generate_update_sql_multiple() {
        let sql = generate_update_sql(10, "products", vec![("price", "99"), ("stock", "5")]);
        let expected = "UPDATE products SET price = '99', stock = '5' WHERE id = 10;";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_generate_update_sql_escapes_quotes() {
        let sql = generate_update_sql(7, "authors", vec![("name", "O'Reilly")]);
        let expected = "UPDATE authors SET name = 'O''Reilly' WHERE id = 7;";
        assert_eq!(sql, expected);
    }

    #[test]
    fn test_sanitize() {
        // No quotes → unchanged
        assert_eq!(sanitize("hello"), "hello");
        // Single quote → doubled
        assert_eq!(sanitize("O'Reilly"), "O''Reilly");
        // Multiple quotes → all doubled
        assert_eq!(sanitize("a'b'c"), "a''b''c");
        // Empty string → empty
        assert_eq!(sanitize(""), "");
    }
}
