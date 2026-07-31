use justino_core::vm::Value;
use justino_stdlib::error::StdlibError;
use justino_stdlib::json::JsonSerializer;

#[test]
fn test_json_parse_and_stringify() -> Result<(), StdlibError> {
    let json_str = r#"{"name":"Justino","version":1,"active":true}"#;

    let val = JsonSerializer::parse(json_str)?;
    if let Value::StructInstance { fields, .. } = &val {
        let f = fields.borrow();
        assert_eq!(f.get("name").unwrap().to_string(), "Justino");
        assert_eq!(f.get("version").unwrap().to_string(), "1");
        assert_eq!(f.get("active").unwrap().to_string(), "true");
    } else {
        panic!("Expected struct instance from JSON parse");
    }

    let stringified = JsonSerializer::stringify(&val);
    assert!(stringified.contains("\"name\":\"Justino\""));
    Ok(())
}
