//! Register-Allocating Bytecode Compiler for the Justino language.

use crate::compiler::opcode::{CompiledFunction, Opcode};
use crate::error::JustinoError;
use crate::parser::ast::*;
use crate::span::Span;
use crate::vm::value::Value;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Local {
    name: String,
    register: u8,
    depth: usize,
}

/// AST -> Register Bytecode compiler.
pub struct Compiler {
    pub file_id: usize,
    function_stack: Vec<CompiledFunction>,
    locals_stack: Vec<Vec<Local>>,
    scope_depth_stack: Vec<usize>,
    next_reg_stack: Vec<u8>,
    top_level_functions: HashMap<String, Rc<CompiledFunction>>,
}

impl Compiler {
    pub fn new(file_id: usize) -> Self {
        let main_func = CompiledFunction::new("<main>", 0);
        Self {
            file_id,
            function_stack: vec![main_func],
            locals_stack: vec![Vec::new()],
            scope_depth_stack: vec![0],
            next_reg_stack: vec![0],
            top_level_functions: HashMap::new(),
        }
    }

    /// Compiles a complete AST Program into a top-level CompiledFunction.
    pub fn compile_program(mut self, program: &Program) -> Result<CompiledFunction, JustinoError> {
        for stmt in &program.stmts {
            self.compile_statement(stmt)?;
        }

        // Implicit return null at end of main function if no explicit return
        let last_is_ret = self
            .current_func()?
            .instructions
            .last()
            .map(|op| matches!(op, Opcode::Return { .. }))
            .unwrap_or(false);

        if !last_is_ret {
            let dummy_span = program.span;
            let ret_reg = self.alloc_reg(dummy_span)?;
            self.current_func_mut()?.emit(Opcode::LoadNull { dst: ret_reg }, dummy_span);
            self.current_func_mut()?.emit(Opcode::Return { src: Some(ret_reg) }, dummy_span);
        }

        let main = self.function_stack.pop().ok_or_else(|| JustinoError::CompileError {
            message: "Compiler stack underflow".to_string(),
            span: program.span,
        })?;

        Ok(main)
    }

    fn compile_statement(&mut self, stmt: &Stmt) -> Result<(), JustinoError> {
        match stmt {
            Stmt::Let {
                name,
                initializer,
                span,
                ..
            } => {
                let init_reg = self.compile_expression(initializer)?;
                let local_reg = self.alloc_reg(*span)?;

                self.current_func_mut()?.emit(
                    Opcode::Move {
                        dst: local_reg,
                        src: init_reg,
                    },
                    *span,
                );

                self.free_reg(init_reg);

                let depth = *self.scope_depth_stack.last().unwrap_or(&0);
                if let Some(locals) = self.locals_stack.last_mut() {
                    locals.push(Local {
                        name: name.clone(),
                        register: local_reg,
                        depth,
                    });
                }
            }
            Stmt::Assignment { target, value, span } => {
                let val_reg = self.compile_expression(value)?;
                match target {
                    Expr::Variable(name, _) => {
                        let target_reg = self.resolve_variable(name, *span)?;
                        self.current_func_mut()?.emit(
                            Opcode::Move {
                                dst: target_reg,
                                src: val_reg,
                            },
                            *span,
                        );
                    }
                    Expr::MemberAccess { object, member, .. } => {
                        let obj_reg = self.compile_expression(object)?;
                        let member_idx = self.current_func_mut()?.add_constant(Value::String(Rc::new(member.clone())));
                        self.current_func_mut()?.emit(
                            Opcode::SetField {
                                obj_reg,
                                field_idx: member_idx,
                                val_reg,
                            },
                            *span,
                        );
                        self.free_reg(obj_reg);
                    }
                    _ => {
                        return Err(JustinoError::CompileError {
                            message: "Invalid assignment target".to_string(),
                            span: *span,
                        });
                    }
                }
                self.free_reg(val_reg);
            }
            Stmt::FunctionDef {
                name,
                params,
                body,
                span,
                ..
            } => {
                let arity = params.len() as u8;
                let sub_compiler_func = CompiledFunction::new(name, arity);
                
                // Set up inner function stack state
                self.function_stack.push(sub_compiler_func);
                self.locals_stack.push(Vec::new());
                self.scope_depth_stack.push(1);
                self.next_reg_stack.push(0);

                // Reserve registers for params (r0, r1, ...)
                for param in params {
                    let reg = self.alloc_reg(param.span)?;
                    if let Some(locals) = self.locals_stack.last_mut() {
                        locals.push(Local {
                            name: param.name.clone(),
                            register: reg,
                            depth: 1,
                        });
                    }
                }

                // Compile function body
                for body_stmt in &body.stmts {
                    self.compile_statement(body_stmt)?;
                }

                // Ensure return instruction
                let is_ret = self
                    .current_func()?
                    .instructions
                    .last()
                    .map(|op| matches!(op, Opcode::Return { .. }))
                    .unwrap_or(false);

                if !is_ret {
                    let null_reg = self.alloc_reg(body.span)?;
                    self.current_func_mut()?.emit(Opcode::LoadNull { dst: null_reg }, body.span);
                    self.current_func_mut()?.emit(Opcode::Return { src: Some(null_reg) }, body.span);
                }

                // Pop compiled function
                let raw_func = self.function_stack.pop().ok_or_else(|| JustinoError::CompileError {
                    message: "Function stack underflow".to_string(),
                    span: *span,
                })?;
                let compiled_func = Rc::new(raw_func);
                self.locals_stack.pop();
                self.scope_depth_stack.pop();
                self.next_reg_stack.pop();

                // Register top level function for cross-function calls
                self.top_level_functions.insert(name.clone(), compiled_func.clone());

                // Add compiled function to parent function's constant pool
                let func_val = Value::Function(compiled_func);
                let const_idx = self.current_func_mut()?.add_constant(func_val);

                let func_reg = self.alloc_reg(*span)?;
                self.current_func_mut()?.emit(
                    Opcode::LoadConst {
                        dst: func_reg,
                        const_idx,
                    },
                    *span,
                );

                // Bind function name to register in current scope
                let depth = *self.scope_depth_stack.last().unwrap_or(&0);
                if let Some(locals) = self.locals_stack.last_mut() {
                    locals.push(Local {
                        name: name.clone(),
                        register: func_reg,
                        depth,
                    });
                }
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
                span,
            } => {
                let cond_reg = self.compile_expression(condition)?;
                
                // Emit placeholder JumpIfFalse
                let jump_false_ip = self.current_func_mut()?.emit(
                    Opcode::JumpIfFalse {
                        condition: cond_reg,
                        offset: 0,
                    },
                    *span,
                );
                self.free_reg(cond_reg);

                self.begin_scope();
                for s in &then_branch.stmts {
                    self.compile_statement(s)?;
                }
                self.end_scope(*span);

                if let Some(else_b) = else_branch {
                    // Emit placeholder Jump to skip else branch after then branch
                    let jump_end_ip = self.current_func_mut()?.emit(Opcode::Jump { offset: 0 }, *span);

                    // Patch jump_false_ip to target start of else branch
                    let else_start_ip = self.current_func()?.instructions.len();
                    let false_offset = (else_start_ip as isize - (jump_false_ip + 1) as isize) as i16;
                    self.current_func_mut()?.instructions[jump_false_ip] = Opcode::JumpIfFalse {
                        condition: cond_reg,
                        offset: false_offset,
                    };

                    self.begin_scope();
                    for s in &else_b.stmts {
                        self.compile_statement(s)?;
                    }
                    self.end_scope(*span);

                    // Patch jump_end_ip
                    let end_ip = self.current_func()?.instructions.len();
                    let end_offset = (end_ip as isize - (jump_end_ip + 1) as isize) as i16;
                    self.current_func_mut()?.instructions[jump_end_ip] = Opcode::Jump { offset: end_offset };
                } else {
                    // Patch jump_false_ip to target end of if
                    let end_ip = self.current_func()?.instructions.len();
                    let false_offset = (end_ip as isize - (jump_false_ip + 1) as isize) as i16;
                    self.current_func_mut()?.instructions[jump_false_ip] = Opcode::JumpIfFalse {
                        condition: cond_reg,
                        offset: false_offset,
                    };
                }
            }
            Stmt::While { condition, body, span } => {
                let start_ip = self.current_func()?.instructions.len();
                let cond_reg = self.compile_expression(condition)?;

                let jump_false_ip = self.current_func_mut()?.emit(
                    Opcode::JumpIfFalse {
                        condition: cond_reg,
                        offset: 0,
                    },
                    *span,
                );
                self.free_reg(cond_reg);

                self.begin_scope();
                for s in &body.stmts {
                    self.compile_statement(s)?;
                }
                self.end_scope(*span);

                // Backward jump to start_ip
                let current_ip = self.current_func()?.instructions.len();
                let loop_offset = (start_ip as isize - (current_ip + 1) as isize) as i16;
                self.current_func_mut()?.emit(Opcode::Jump { offset: loop_offset }, *span);

                // Patch jump_false_ip
                let end_ip = self.current_func()?.instructions.len();
                let false_offset = (end_ip as isize - (jump_false_ip + 1) as isize) as i16;
                self.current_func_mut()?.instructions[jump_false_ip] = Opcode::JumpIfFalse {
                    condition: cond_reg,
                    offset: false_offset,
                };
            }
            Stmt::Return { value, span } => {
                let src_reg = if let Some(expr) = value {
                    Some(self.compile_expression(expr)?)
                } else {
                    None
                };
                self.current_func_mut()?.emit(Opcode::Return { src: src_reg }, *span);
                if let Some(r) = src_reg {
                    self.free_reg(r);
                }
            }
            Stmt::Expr(expr) => {
                let reg = self.compile_expression(expr)?;
                self.free_reg(reg);
            }
            Stmt::Block(block) => {
                self.begin_scope();
                for s in &block.stmts {
                    self.compile_statement(s)?;
                }
                self.end_scope(block.span);
            }
            Stmt::StructDef { .. } => {
                // Struct definitions are type declarations, handled at runtime via StructInit
            }
        }

        Ok(())
    }

    /// Compiles an expression into a register and returns the register index.
    fn compile_expression(&mut self, expr: &Expr) -> Result<u8, JustinoError> {
        match expr {
            Expr::Literal(lit, s) => {
                let target_reg = self.alloc_reg(*s)?;
                match lit {
                    Literal::Int(val) => {
                        let idx = self.current_func_mut()?.add_constant(Value::Int(*val));
                        self.current_func_mut()?.emit(Opcode::LoadConst { dst: target_reg, const_idx: idx }, *s);
                    }
                    Literal::Float(val) => {
                        let idx = self.current_func_mut()?.add_constant(Value::Float(*val));
                        self.current_func_mut()?.emit(Opcode::LoadConst { dst: target_reg, const_idx: idx }, *s);
                    }
                    Literal::Bool(val) => {
                        self.current_func_mut()?.emit(Opcode::LoadBool { dst: target_reg, val: *val }, *s);
                    }
                    Literal::String(val) => {
                        let idx = self.current_func_mut()?.add_constant(Value::String(Rc::new(val.clone())));
                        self.current_func_mut()?.emit(Opcode::LoadConst { dst: target_reg, const_idx: idx }, *s);
                    }
                    Literal::Null => {
                        self.current_func_mut()?.emit(Opcode::LoadNull { dst: target_reg }, *s);
                    }
                }
                Ok(target_reg)
            }
            Expr::Variable(name, s) => {
                let src_reg = self.resolve_variable(name, *s)?;
                let dst_reg = self.alloc_reg(*s)?;
                self.current_func_mut()?.emit(Opcode::Move { dst: dst_reg, src: src_reg }, *s);
                Ok(dst_reg)
            }
            Expr::Binary { op, left, right, span } => {
                let lhs = self.compile_expression(left)?;
                let rhs = self.compile_expression(right)?;
                let dst = self.alloc_reg(*span)?;

                let opcode = match op {
                    BinaryOp::Add => Opcode::Add { dst, lhs, rhs },
                    BinaryOp::Sub => Opcode::Sub { dst, lhs, rhs },
                    BinaryOp::Mul => Opcode::Mul { dst, lhs, rhs },
                    BinaryOp::Div => Opcode::Div { dst, lhs, rhs },
                    BinaryOp::Mod => Opcode::Mod { dst, lhs, rhs },
                    BinaryOp::Equal => Opcode::Equal { dst, lhs, rhs },
                    BinaryOp::NotEqual => Opcode::NotEqual { dst, lhs, rhs },
                    BinaryOp::Less => Opcode::LessThan { dst, lhs, rhs },
                    BinaryOp::Greater => Opcode::GreaterThan { dst, lhs, rhs },
                    BinaryOp::LessEqual => Opcode::LessEqual { dst, lhs, rhs },
                    BinaryOp::GreaterEqual => Opcode::GreaterEqual { dst, lhs, rhs },
                    BinaryOp::And => Opcode::Mul { dst, lhs, rhs }, // Boolean logical and represented via arithmetic or bit logic
                    BinaryOp::Or => Opcode::Add { dst, lhs, rhs },
                };

                self.current_func_mut()?.emit(opcode, *span);
                self.free_reg(rhs);
                self.free_reg(lhs);
                Ok(dst)
            }
            Expr::Unary { op, operand, span } => {
                let src = self.compile_expression(operand)?;
                let dst = self.alloc_reg(*span)?;
                let opcode = match op {
                    UnaryOp::Negate => Opcode::Neg { dst, src },
                    UnaryOp::Not => Opcode::Not { dst, src },
                };
                self.current_func_mut()?.emit(opcode, *span);
                self.free_reg(src);
                Ok(dst)
            }
            Expr::Call { callee, args, span } => {
                let func_reg = self.compile_expression(callee)?;
                
                // Allocate contiguous registers for arguments
                let arg_count = args.len() as u8;
                let mut arg_regs = Vec::new();
                for arg in args {
                    let r = self.compile_expression(arg)?;
                    arg_regs.push(r);
                }

                let arg_start = if let Some(first) = arg_regs.first() {
                    *first
                } else {
                    0
                };

                let dst = self.alloc_reg(*span)?;
                self.current_func_mut()?.emit(
                    Opcode::Call {
                        dst,
                        func_reg,
                        arg_start,
                        arg_count,
                    },
                    *span,
                );

                for r in arg_regs.into_iter().rev() {
                    self.free_reg(r);
                }
                self.free_reg(func_reg);

                Ok(dst)
            }
            Expr::StructInit { name, fields, span } => {
                let name_idx = self.current_func_mut()?.add_constant(Value::String(Rc::new(name.clone())));
                let dst = self.alloc_reg(*span)?;

                self.current_func_mut()?.emit(
                    Opcode::NewStruct {
                        dst,
                        name_idx,
                        field_count: fields.len() as u8,
                    },
                    *span,
                );

                for (field_name, field_expr) in fields {
                    let val_reg = self.compile_expression(field_expr)?;
                    let f_idx = self.current_func_mut()?.add_constant(Value::String(Rc::new(field_name.clone())));
                    self.current_func_mut()?.emit(
                        Opcode::SetField {
                            obj_reg: dst,
                            field_idx: f_idx,
                            val_reg,
                        },
                        *span,
                    );
                    self.free_reg(val_reg);
                }

                Ok(dst)
            }
            Expr::MemberAccess { object, member, span } => {
                let obj_reg = self.compile_expression(object)?;
                let field_idx = self.current_func_mut()?.add_constant(Value::String(Rc::new(member.clone())));
                let dst = self.alloc_reg(*span)?;

                self.current_func_mut()?.emit(
                    Opcode::GetField {
                        dst,
                        obj_reg,
                        field_idx,
                    },
                    *span,
                );

                self.free_reg(obj_reg);
                Ok(dst)
            }
            Expr::Spawn { expr, span } => {
                let func_reg = self.compile_expression(expr)?;
                self.current_func_mut()?.emit(Opcode::Spawn { func_reg }, *span);
                let dst = self.alloc_reg(*span)?;
                self.current_func_mut()?.emit(Opcode::LoadNull { dst }, *span);
                self.free_reg(func_reg);
                Ok(dst)
            }
            Expr::Await { expr, span: _ } => {
                // In Phase 1 VM, await evaluates the target expression directly
                self.compile_expression(expr)
            }
            Expr::InterpolatedString { parts, span } => {
                let mut part_regs = Vec::new();
                for part in parts {
                    let r = self.compile_expression(part)?;
                    part_regs.push(r);
                }

                let start_reg = part_regs.first().copied().unwrap_or(0);
                let count = part_regs.len() as u8;
                let dst = self.alloc_reg(*span)?;

                self.current_func_mut()?.emit(
                    Opcode::ConcatStrings {
                        dst,
                        start_reg,
                        count,
                    },
                    *span,
                );

                for r in part_regs.into_iter().rev() {
                    self.free_reg(r);
                }

                Ok(dst)
            }
        }
    }

    fn resolve_variable(&mut self, name: &str, span: Span) -> Result<u8, JustinoError> {
        if let Some(locals) = self.locals_stack.last() {
            for local in locals.iter().rev() {
                if local.name == name {
                    return Ok(local.register);
                }
            }
        }

        if let Some(func_rc) = self.top_level_functions.get(name).cloned() {
            let func_val = Value::Function(func_rc);
            let const_idx = self.current_func_mut()?.add_constant(func_val);
            let reg = self.alloc_reg(span)?;
            self.current_func_mut()?.emit(Opcode::LoadConst { dst: reg, const_idx }, span);
            return Ok(reg);
        }

        Err(JustinoError::CompileError {
            message: format!("Undefined variable '{}'", name),
            span,
        })
    }

    fn begin_scope(&mut self) {
        if let Some(depth) = self.scope_depth_stack.last_mut() {
            *depth += 1;
        }
    }

    fn end_scope(&mut self, _span: Span) {
        let current_depth = *self.scope_depth_stack.last().unwrap_or(&0);
        let mut to_free = Vec::new();
        if let Some(locals) = self.locals_stack.last_mut() {
            while let Some(local) = locals.last() {
                if local.depth >= current_depth {
                    to_free.push(local.register);
                    locals.pop();
                } else {
                    break;
                }
            }
        }
        for reg in to_free {
            self.free_reg(reg);
        }
        if let Some(depth) = self.scope_depth_stack.last_mut() {
            if *depth > 0 {
                *depth -= 1;
            }
        }
    }

    fn alloc_reg(&mut self, span: Span) -> Result<u8, JustinoError> {
        let next_reg = self.next_reg_stack.last_mut().ok_or_else(|| JustinoError::CompileError {
            message: "Register stack error".to_string(),
            span,
        })?;

        if *next_reg == 255 {
            return Err(JustinoError::CompileError {
                message: "Register limit exceeded (max 256 registers per frame)".to_string(),
                span,
            });
        }

        let reg = *next_reg;
        *next_reg += 1;

        let func = self.function_stack.last_mut().ok_or_else(|| JustinoError::CompileError {
            message: "Function stack underflow".to_string(),
            span,
        })?;

        if *next_reg > func.num_registers {
            func.num_registers = *next_reg;
        }

        Ok(reg)
    }

    fn free_reg(&mut self, reg: u8) {
        if let Some(next_reg) = self.next_reg_stack.last_mut() {
            if reg + 1 == *next_reg {
                *next_reg = reg;
            }
        }
    }

    fn current_func(&self) -> Result<&CompiledFunction, JustinoError> {
        self.function_stack.last().ok_or_else(|| JustinoError::CompileError {
            message: "Function stack is empty".to_string(),
            span: Span::dummy(),
        })
    }

    fn current_func_mut(&mut self) -> Result<&mut CompiledFunction, JustinoError> {
        self.function_stack.last_mut().ok_or_else(|| JustinoError::CompileError {
            message: "Function stack is empty".to_string(),
            span: Span::dummy(),
        })
    }
}
