use justino_core::lexer::Scanner;
use justino_core::parser::ast::*;
use justino_core::parser::Parser;
use justino_core::JustinoError;

#[test]
fn test_parser_pratt_precedence() -> Result<(), JustinoError> {
    let source = "let res = 2 + 3 * 4;";
    let mut scanner = Scanner::new(source, 1);
    let tokens = scanner.scan()?;

    let mut parser = Parser::new(tokens, 1);
    let program = parser.parse_program()?;

    assert_eq!(program.stmts.len(), 1);
    if let Stmt::Let { initializer, .. } = &program.stmts[0] {
        if let Expr::Binary { op, left, right, .. } = initializer {
            assert_eq!(*op, BinaryOp::Add);
            assert!(matches!(left.as_ref(), Expr::Literal(Literal::Int(2), _)));
            if let Expr::Binary { op: inner_op, .. } = right.as_ref() {
                assert_eq!(*inner_op, BinaryOp::Mul);
            } else {
                panic!("Expected binary operation for right operand");
            }
        } else {
            panic!("Expected binary operation for initializer");
        }
    } else {
        panic!("Expected let statement");
    }

    Ok(())
}

#[test]
fn test_parser_function_def() -> Result<(), JustinoError> {
    let source = r#"
        fn add(a: int, b: int) -> int {
            return a + b;
        }
    "#;

    let mut scanner = Scanner::new(source, 1);
    let tokens = scanner.scan()?;

    let mut parser = Parser::new(tokens, 1);
    let program = parser.parse_program()?;

    assert_eq!(program.stmts.len(), 1);
    if let Stmt::FunctionDef { name, params, .. } = &program.stmts[0] {
        assert_eq!(name, "add");
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "a");
        assert_eq!(params[1].name, "b");
    } else {
        panic!("Expected function definition statement");
    }

    Ok(())
}
