use justino_core::eval_jucode;
use justino_core::vm::Value;
use justino_stdlib::db::SqliteDatabase;
use justino_stdlib::error::StdlibError;
use justino_stdlib::fs::{AsyncFile, EnvReader};
use justino_stdlib::i18n::CldrFormatter;

#[test]
fn test_stdlib_full_e2e_app_desktop() -> Result<(), StdlibError> {
    // 1. Env Reader
    let env_content = "APP_PORT=8080\nDB_FILE=target/integration.db\n";
    AsyncFile::write_file("target/app.env", env_content)?;
    let env_vars = EnvReader::parse_env_file("target/app.env")?;
    assert_eq!(env_vars.get("APP_PORT").unwrap(), "8080");

    // 2. Database Open & Query
    let db_file = env_vars.get("DB_FILE").unwrap();
    let _ = std::fs::remove_file(db_file);
    let db = SqliteDatabase::open(db_file)?;
    db.query("CREATE TABLE products (id INT, price FLOAT)", &[])?;
    db.query("INSERT INTO products VALUES (?, ?)", &[Value::Int(101), Value::Float(1250.50)])?;
    let rows = db.query("SELECT * FROM products", &[])?;
    assert_eq!(rows.len(), 1);

    // 3. CLDR Currency Formatting
    let formatted_brl = CldrFormatter::format_currency(1250.50, "BRL");
    let formatted_usd = CldrFormatter::format_currency(1250.50, "USD");
    assert_eq!(formatted_brl, "R$ 1250,50");
    assert_eq!(formatted_usd, "$1250.50");

    // 4. Executing .jucode logic in VM
    let jucode_source = r#"
        struct Product {
            id: int,
            name: string,
            price: int
        }

        fn create_product() -> Product {
            return Product {
                id: 101,
                name: "Justino Workstation",
                price: 1250
            };
        }

        fn main() -> int {
            let p = create_product();
            return p.price;
        }

        return main();
    "#;

    let res = eval_jucode(jucode_source, 1)?;
    assert_eq!(res, Value::Int(1250));

    Ok(())
}
