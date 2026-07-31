use justino_core::compiler::opcode::Opcode;
use justino_core::compiler::Compiler;
use justino_core::lexer::Scanner;
use justino_core::parser::Parser;
use justino_core::JustinoError;

#[test]
fn test_compiler_emits_register_opcodes() -> Result<(), JustinoError> {
    let source = "let x = 10 + 20;";
    let mut scanner = Scanner::new(source, 1);
    let tokens = scanner.scan()?;

    let mut parser = Parser::new(tokens, 1);
    let program = parser.parse_program()?;

    let compiler = Compiler::new(1);
    let compiled_func = compiler.compile_program(&program)?;

    assert!(compiled_func.instructions.iter().any(|op| matches!(op, Opcode::Add { .. })));
    assert!(compiled_func.num_registers >= 2);

    Ok(())
}
