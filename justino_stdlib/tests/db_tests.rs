use justino_core::vm::Value;
use justino_stdlib::db::SqliteDatabase;
use justino_stdlib::error::StdlibError;
use std::rc::Rc;

#[test]
fn test_sqlite_database_crud_operations() -> Result<(), StdlibError> {
    let db_path = "target/test_app.db";
    let _ = std::fs::remove_file(db_path);
    let db = SqliteDatabase::open(db_path)?;

    // CREATE TABLE
    db.query("CREATE TABLE users (id INT, name TEXT)", &[])?;

    // INSERT INTO
    let param_id = Value::Int(1);
    let param_name = Value::String(Rc::new("Alice Developer".to_string()));
    db.query("INSERT INTO users VALUES (?, ?)", &[param_id, param_name])?;

    // SELECT
    let rows = db.query("SELECT * FROM users", &[])?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get("col_1").unwrap().to_string(), "1");
    assert_eq!(rows[0].get("col_2").unwrap().to_string(), "Alice Developer");

    Ok(())
}
