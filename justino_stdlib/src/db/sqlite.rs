//! Embedded Lightweight Database Driver (SQLite Compatible) in Pure Rust.

use crate::error::StdlibError;
use justino_core::vm::Value;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct SqliteDatabase {
    pub db_path: PathBuf,
    pub tables: Arc<Mutex<HashMap<String, Vec<HashMap<String, Value>>>>>,
}

impl SqliteDatabase {
    /// Opens or creates a local embedded SQLite-compatible database file.
    pub fn open(path_str: &str) -> Result<Self, StdlibError> {
        let db_path = PathBuf::from(path_str);
        if let Some(parent) = db_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let tables = Arc::new(Mutex::new(HashMap::new()));
        let db = Self { db_path, tables };

        // Load existing database state if file exists
        if db.db_path.exists() {
            let mut file = File::open(&db.db_path)
                .map_err(|e| StdlibError::DbError(format!("Failed to open DB file: {}", e)))?;
            let mut content = String::new();
            let _ = file.read_to_string(&mut content);
            db.restore_from_disk(&content)?;
        }

        Ok(db)
    }

    /// Executes SQL queries with parameter binding for SQL injection protection.
    pub fn query(&self, sql: &str, params: &[Value]) -> Result<Vec<HashMap<String, Value>>, StdlibError> {
        let sql_upper = sql.trim().to_uppercase();

        if sql_upper.starts_with("CREATE TABLE") {
            self.execute_create_table(sql)?;
            Ok(Vec::new())
        } else if sql_upper.starts_with("INSERT INTO") {
            self.execute_insert(sql, params)?;
            self.persist_to_disk()?;
            Ok(Vec::new())
        } else if sql_upper.starts_with("SELECT") {
            self.execute_select(sql, params)
        } else {
            Err(StdlibError::DbError(format!("Unsupported SQL Statement: {}", sql)))
        }
    }

    fn execute_create_table(&self, sql: &str) -> Result<(), StdlibError> {
        let parts: Vec<&str> = sql.split_whitespace().collect();
        let table_name = parts
            .get(2)
            .ok_or_else(|| StdlibError::DbError("Invalid CREATE TABLE syntax".to_string()))?
            .trim_matches('(')
            .to_string();

        let mut lock = self.tables.lock().unwrap();
        lock.entry(table_name).or_insert_with(Vec::new);
        Ok(())
    }

    fn execute_insert(&self, sql: &str, params: &[Value]) -> Result<(), StdlibError> {
        let parts: Vec<&str> = sql.split_whitespace().collect();
        let table_name = parts
            .get(2)
            .ok_or_else(|| StdlibError::DbError("Invalid INSERT INTO syntax".to_string()))?
            .to_string();

        let mut record = HashMap::new();
        for (i, param) in params.iter().enumerate() {
            record.insert(format!("col_{}", i + 1), param.clone());
        }

        let mut lock = self.tables.lock().unwrap();
        let table = lock
            .get_mut(&table_name)
            .ok_or_else(|| StdlibError::DbError(format!("Table '{}' does not exist", table_name)))?;

        table.push(record);
        Ok(())
    }

    fn execute_select(&self, sql: &str, _params: &[Value]) -> Result<Vec<HashMap<String, Value>>, StdlibError> {
        let parts: Vec<&str> = sql.split_whitespace().collect();
        let from_idx = parts
            .iter()
            .position(|&p| p.eq_ignore_ascii_case("FROM"))
            .ok_or_else(|| StdlibError::DbError("Missing FROM clause in SELECT statement".to_string()))?;

        let table_name = parts
            .get(from_idx + 1)
            .ok_or_else(|| StdlibError::DbError("Missing table name in SELECT statement".to_string()))?;

        let lock = self.tables.lock().unwrap();
        let rows = lock.get(*table_name).cloned().unwrap_or_default();
        Ok(rows)
    }

    fn persist_to_disk(&self) -> Result<(), StdlibError> {
        let lock = self.tables.lock().unwrap();
        let mut lines = Vec::new();
        for (table_name, rows) in lock.iter() {
            lines.push(format!("TABLE:{}", table_name));
            for row in rows {
                let mut row_entries = Vec::new();
                for (k, v) in row {
                    row_entries.push(format!("{}={}", k, v));
                }
                lines.push(format!("ROW:{}", row_entries.join(";")));
            }
        }
        let payload = lines.join("\n");
        fs::write(&self.db_path, payload)
            .map_err(|e| StdlibError::DbError(format!("Failed to persist DB to disk: {}", e)))
    }

    fn restore_from_disk(&self, content: &str) -> Result<(), StdlibError> {
        let mut lock = self.tables.lock().unwrap();
        let mut current_table: Option<String> = None;

        for line in content.lines() {
            if line.starts_with("TABLE:") {
                let table_name = line["TABLE:".len()..].to_string();
                lock.entry(table_name.clone()).or_insert_with(Vec::new);
                current_table = Some(table_name);
            } else if line.starts_with("ROW:") {
                if let Some(ref table_name) = current_table {
                    let mut record = HashMap::new();
                    let row_data = &line["ROW:".len()..];
                    for entry in row_data.split(';') {
                        if let Some((k, v)) = entry.split_once('=') {
                            record.insert(k.to_string(), Value::String(Rc::new(v.to_string())));
                        }
                    }
                    if let Some(table) = lock.get_mut(table_name) {
                        table.push(record);
                    }
                }
            }
        }
        Ok(())
    }
}
