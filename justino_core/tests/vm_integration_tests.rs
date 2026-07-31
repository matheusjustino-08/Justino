use justino_core::eval_jucode;
use justino_core::vm::Value;
use justino_core::JustinoError;

#[test]
fn test_vm_e2e_jucode_execution() -> Result<(), JustinoError> {
    let jucode_source = r#"
        struct Point {
            x: int,
            y: int
        }

        fn calculate_sum(limit: int) -> int {
            let mut total = 0;
            let mut i = 1;
            while i <= limit {
                total = total + i;
                i = i + 1;
            }
            return total;
        }

        fn execute_test() -> int {
            let p = Point { x: 10, y: 20 };
            let sum = calculate_sum(5); // 1 + 2 + 3 + 4 + 5 = 15
            if sum == 15 {
                return p.x + p.y + sum; // 10 + 20 + 15 = 45
            } else {
                return 0;
            }
        }

        return execute_test();
    "#;

    let result = eval_jucode(jucode_source, 1)?;
    assert_eq!(result, Value::Int(45));

    Ok(())
}

#[test]
fn test_vm_string_interpolation_e2e() -> Result<(), JustinoError> {
    let jucode_source = r#"
        let user = "Justino";
        let version = 1;
        let message = "Welcome to ${user} v${version}!";
        return message;
    "#;

    let result = eval_jucode(jucode_source, 1)?;
    if let Value::String(s) = result {
        assert_eq!(s.as_str(), "Welcome to Justino v1!");
    } else {
        panic!("Expected string value from interpolation");
    }

    Ok(())
}
