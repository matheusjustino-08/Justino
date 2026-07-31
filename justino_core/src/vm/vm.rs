//! High-performance Register-Based Virtual Machine for the Justino language.

use crate::compiler::opcode::{CompiledFunction, Opcode};
use crate::error::JustinoError;
use crate::span::Span;
use crate::vm::gc::GcArena;
use crate::vm::value::Value;
use std::collections::HashMap;
use std::rc::Rc;

/// Active CallFrame representing a function invocation context.
#[derive(Debug, Clone)]
pub struct CallFrame {
    pub function: Rc<CompiledFunction>,
    pub ip: usize,
    pub registers: Vec<Value>,
    pub return_reg: Option<u8>,
}

impl CallFrame {
    pub fn new(function: Rc<CompiledFunction>, return_reg: Option<u8>) -> Self {
        let num_regs = (function.num_registers as usize).max(1);
        Self {
            function,
            ip: 0,
            registers: vec![Value::Null; num_regs],
            return_reg,
        }
    }
}

/// High-performance Register-Based Virtual Machine.
pub struct VM {
    frames: Vec<CallFrame>,
    gc: GcArena,
    pub globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            gc: GcArena::new(),
            globals: HashMap::new(),
        }
    }

    /// Returns a reference to the GC arena.
    pub fn gc(&self) -> &GcArena {
        &self.gc
    }

    /// Returns a mutable reference to the GC arena.
    pub fn gc_mut(&mut self) -> &mut GcArena {
        &mut self.gc
    }

    /// Executes a top-level CompiledFunction and returns the final Result Value.
    pub fn interpret(&mut self, main_func: CompiledFunction) -> Result<Value, JustinoError> {
        let main_rc = Rc::new(main_func);
        let root_frame = CallFrame::new(main_rc, None);
        self.frames.clear();
        self.frames.push(root_frame);

        self.run()
    }

    /// Main Fetch-Decode-Execute dispatch loop.
    fn run(&mut self) -> Result<Value, JustinoError> {
        while !self.frames.is_empty() {
            let frame_idx = self.frames.len() - 1;
            let ip = self.frames[frame_idx].ip;
            let instructions_len = self.frames[frame_idx].function.instructions.len();

            if ip >= instructions_len {
                // End of function reached implicitly
                let ret_val = Value::Null;
                let popped_frame = self.frames.pop().ok_or_else(|| JustinoError::RuntimeError {
                    message: "Call stack underflow".to_string(),
                    span: None,
                })?;

                if self.frames.is_empty() {
                    return Ok(ret_val);
                }

                if let Some(dst_reg) = popped_frame.return_reg {
                    let top_idx = self.frames.len() - 1;
                    self.set_reg(top_idx, dst_reg, ret_val, Span::dummy())?;
                }
                continue;
            }

            let op = self.frames[frame_idx].function.instructions[ip].clone();
            let span = self.frames[frame_idx]
                .function
                .spans
                .get(ip)
                .copied()
                .unwrap_or_else(Span::dummy);

            self.frames[frame_idx].ip += 1;

            match op {
                Opcode::LoadConst { dst, const_idx } => {
                    let const_val = self.frames[frame_idx]
                        .function
                        .constants
                        .get(const_idx as usize)
                        .cloned()
                        .ok_or_else(|| JustinoError::RuntimeError {
                            message: format!("Constant pool index out of bounds: {}", const_idx),
                            span: Some(span),
                        })?;
                    self.set_reg(frame_idx, dst, const_val, span)?;
                }
                Opcode::Move { dst, src } => {
                    let val = self.get_reg(frame_idx, src, span)?.clone();
                    self.set_reg(frame_idx, dst, val, span)?;
                }
                Opcode::LoadNull { dst } => {
                    self.set_reg(frame_idx, dst, Value::Null, span)?;
                }
                Opcode::LoadBool { dst, val } => {
                    self.set_reg(frame_idx, dst, Value::Bool(val), span)?;
                }
                Opcode::Add { dst, lhs, rhs } => {
                    let left = self.get_reg(frame_idx, lhs, span)?.clone();
                    let right = self.get_reg(frame_idx, rhs, span)?.clone();

                    let res = match (left, right) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a.wrapping_add(b)),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                        (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 + b),
                        (Value::Float(a), Value::Int(b)) => Value::Float(a + b as f64),
                        (Value::String(a), Value::String(b)) => {
                            self.gc.alloc_string(format!("{}{}", a, b))
                        }
                        (Value::String(a), b) => self.gc.alloc_string(format!("{}{}", a, b)),
                        (a, Value::String(b)) => self.gc.alloc_string(format!("{}{}", a, b)),
                        (l, r) => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot add type '{}' and '{}'", l.type_name(), r.type_name()),
                                span: Some(span),
                            });
                        }
                    };
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::Sub { dst, lhs, rhs } => {
                    let left = self.get_reg(frame_idx, lhs, span)?.clone();
                    let right = self.get_reg(frame_idx, rhs, span)?.clone();

                    let res = match (left, right) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a.wrapping_sub(b)),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
                        (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 - b),
                        (Value::Float(a), Value::Int(b)) => Value::Float(a - b as f64),
                        (l, r) => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot subtract type '{}' and '{}'", l.type_name(), r.type_name()),
                                span: Some(span),
                            });
                        }
                    };
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::Mul { dst, lhs, rhs } => {
                    let left = self.get_reg(frame_idx, lhs, span)?.clone();
                    let right = self.get_reg(frame_idx, rhs, span)?.clone();

                    let res = match (left, right) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a.wrapping_mul(b)),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                        (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 * b),
                        (Value::Float(a), Value::Int(b)) => Value::Float(a * b as f64),
                        (l, r) => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot multiply type '{}' and '{}'", l.type_name(), r.type_name()),
                                span: Some(span),
                            });
                        }
                    };
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::Div { dst, lhs, rhs } => {
                    let left = self.get_reg(frame_idx, lhs, span)?.clone();
                    let right = self.get_reg(frame_idx, rhs, span)?.clone();

                    let res = match (left, right) {
                        (Value::Int(a), Value::Int(b)) => {
                            if b == 0 {
                                return Err(JustinoError::RuntimeError {
                                    message: "Division by zero".to_string(),
                                    span: Some(span),
                                });
                            }
                            Value::Int(a / b)
                        }
                        (Value::Float(a), Value::Float(b)) => {
                            if b == 0.0 {
                                return Err(JustinoError::RuntimeError {
                                    message: "Division by zero".to_string(),
                                    span: Some(span),
                                });
                            }
                            Value::Float(a / b)
                        }
                        (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 / b),
                        (Value::Float(a), Value::Int(b)) => Value::Float(a / b as f64),
                        (l, r) => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot divide type '{}' and '{}'", l.type_name(), r.type_name()),
                                span: Some(span),
                            });
                        }
                    };
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::Mod { dst, lhs, rhs } => {
                    let left = self.get_reg(frame_idx, lhs, span)?.clone();
                    let right = self.get_reg(frame_idx, rhs, span)?.clone();

                    let res = match (left, right) {
                        (Value::Int(a), Value::Int(b)) => {
                            if b == 0 {
                                return Err(JustinoError::RuntimeError {
                                    message: "Modulo by zero".to_string(),
                                    span: Some(span),
                                });
                            }
                            Value::Int(a % b)
                        }
                        (l, r) => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot modulo type '{}' and '{}'", l.type_name(), r.type_name()),
                                span: Some(span),
                            });
                        }
                    };
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::Neg { dst, src } => {
                    let val = self.get_reg(frame_idx, src, span)?.clone();
                    let res = match val {
                        Value::Int(i) => Value::Int(-i),
                        Value::Float(f) => Value::Float(-f),
                        v => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot negate type '{}'", v.type_name()),
                                span: Some(span),
                            });
                        }
                    };
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::Not { dst, src } => {
                    let val = self.get_reg(frame_idx, src, span)?;
                    let res = Value::Bool(!val.is_truthy());
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::Equal { dst, lhs, rhs } => {
                    let left = self.get_reg(frame_idx, lhs, span)?;
                    let right = self.get_reg(frame_idx, rhs, span)?;
                    let res = Value::Bool(left == right);
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::NotEqual { dst, lhs, rhs } => {
                    let left = self.get_reg(frame_idx, lhs, span)?;
                    let right = self.get_reg(frame_idx, rhs, span)?;
                    let res = Value::Bool(left != right);
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::LessThan { dst, lhs, rhs } => {
                    let left = self.get_reg(frame_idx, lhs, span)?;
                    let right = self.get_reg(frame_idx, rhs, span)?;
                    let res = match (left, right) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
                        (Value::Int(a), Value::Float(b)) => Value::Bool((*a as f64) < *b),
                        (Value::Float(a), Value::Int(b)) => Value::Bool(*a < (*b as f64)),
                        (l, r) => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot compare '<' on '{}' and '{}'", l.type_name(), r.type_name()),
                                span: Some(span),
                            });
                        }
                    };
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::GreaterThan { dst, lhs, rhs } => {
                    let left = self.get_reg(frame_idx, lhs, span)?;
                    let right = self.get_reg(frame_idx, rhs, span)?;
                    let res = match (left, right) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
                        (Value::Int(a), Value::Float(b)) => Value::Bool((*a as f64) > *b),
                        (Value::Float(a), Value::Int(b)) => Value::Bool(*a > (*b as f64)),
                        (l, r) => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot compare '>' on '{}' and '{}'", l.type_name(), r.type_name()),
                                span: Some(span),
                            });
                        }
                    };
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::LessEqual { dst, lhs, rhs } => {
                    let left = self.get_reg(frame_idx, lhs, span)?;
                    let right = self.get_reg(frame_idx, rhs, span)?;
                    let res = match (left, right) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
                        (l, r) => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot compare '<=' on '{}' and '{}'", l.type_name(), r.type_name()),
                                span: Some(span),
                            });
                        }
                    };
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::GreaterEqual { dst, lhs, rhs } => {
                    let left = self.get_reg(frame_idx, lhs, span)?;
                    let right = self.get_reg(frame_idx, rhs, span)?;
                    let res = match (left, right) {
                        (Value::Int(a), Value::Int(b)) => Value::Bool(a >= b),
                        (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
                        (l, r) => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot compare '>=' on '{}' and '{}'", l.type_name(), r.type_name()),
                                span: Some(span),
                            });
                        }
                    };
                    self.set_reg(frame_idx, dst, res, span)?;
                }
                Opcode::Jump { offset } => {
                    let new_ip = (self.frames[frame_idx].ip as isize + offset as isize) as usize;
                    self.frames[frame_idx].ip = new_ip;
                }
                Opcode::JumpIfFalse { condition, offset } => {
                    let cond_val = self.get_reg(frame_idx, condition, span)?;
                    if !cond_val.is_truthy() {
                        let new_ip = (self.frames[frame_idx].ip as isize + offset as isize) as usize;
                        self.frames[frame_idx].ip = new_ip;
                    }
                }
                Opcode::Call {
                    dst,
                    func_reg,
                    arg_start,
                    arg_count,
                } => {
                    let callee = self.get_reg(frame_idx, func_reg, span)?.clone();
                    match callee {
                        Value::Function(func) => {
                            if func.arity != arg_count {
                                return Err(JustinoError::RuntimeError {
                                    message: format!(
                                        "Function '{}' expects {} arguments, but {} were provided",
                                        func.name, func.arity, arg_count
                                    ),
                                    span: Some(span),
                                });
                            }

                            let mut new_frame = CallFrame::new(func.clone(), Some(dst));
                            for i in 0..arg_count {
                                let arg_val = self.get_reg(frame_idx, arg_start + i, span)?.clone();
                                new_frame.registers[i as usize] = arg_val;
                            }

                            self.frames.push(new_frame);
                        }
                        other => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot call non-function type '{}'", other.type_name()),
                                span: Some(span),
                            });
                        }
                    }
                }
                Opcode::Return { src } => {
                    let ret_val = if let Some(r) = src {
                        self.get_reg(frame_idx, r, span)?.clone()
                    } else {
                        Value::Null
                    };

                    let popped_frame = self.frames.pop().ok_or_else(|| JustinoError::RuntimeError {
                        message: "Call stack underflow".to_string(),
                        span: Some(span),
                    })?;
                    if self.frames.is_empty() {
                        return Ok(ret_val);
                    }

                    if let Some(dst_reg) = popped_frame.return_reg {
                        let top_idx = self.frames.len() - 1;
                        self.set_reg(top_idx, dst_reg, ret_val, span)?;
                    }
                }
                Opcode::Spawn { func_reg } => {
                    // For Phase 1 VM, spawn evaluates the function target safely
                    let _ = self.get_reg(frame_idx, func_reg, span)?;
                }
                Opcode::NewStruct {
                    dst,
                    name_idx,
                    field_count: _,
                } => {
                    let name_val = self.frames[frame_idx]
                        .function
                        .constants
                        .get(name_idx as usize)
                        .cloned()
                        .ok_or_else(|| JustinoError::RuntimeError {
                            message: "Struct name index out of bounds".to_string(),
                            span: Some(span),
                        })?;

                    let struct_name = match name_val {
                        Value::String(s) => s.as_ref().clone(),
                        _ => "UnknownStruct".to_string(),
                    };

                    let struct_inst = self.gc.alloc_struct(struct_name, HashMap::new());
                    self.set_reg(frame_idx, dst, struct_inst, span)?;
                }
                Opcode::SetField {
                    obj_reg,
                    field_idx,
                    val_reg,
                } => {
                    let field_val = self.frames[frame_idx]
                        .function
                        .constants
                        .get(field_idx as usize)
                        .cloned()
                        .ok_or_else(|| JustinoError::RuntimeError {
                            message: "Field index out of bounds".to_string(),
                            span: Some(span),
                        })?;

                    let field_name = match field_val {
                        Value::String(s) => s.as_ref().clone(),
                        _ => {
                            return Err(JustinoError::RuntimeError {
                                message: "Field name must be a string".to_string(),
                                span: Some(span),
                            });
                        }
                    };

                    let val = self.get_reg(frame_idx, val_reg, span)?.clone();
                    let obj = self.get_reg(frame_idx, obj_reg, span)?;

                    match obj {
                        Value::StructInstance { fields, .. } | Value::Object(fields) => {
                            fields.borrow_mut().insert(field_name, val);
                        }
                        other => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot set field on type '{}'", other.type_name()),
                                span: Some(span),
                            });
                        }
                    }
                }
                Opcode::GetField {
                    dst,
                    obj_reg,
                    field_idx,
                } => {
                    let field_val = self.frames[frame_idx]
                        .function
                        .constants
                        .get(field_idx as usize)
                        .cloned()
                        .ok_or_else(|| JustinoError::RuntimeError {
                            message: "Field index out of bounds".to_string(),
                            span: Some(span),
                        })?;

                    let field_name = match field_val {
                        Value::String(s) => s.as_ref().clone(),
                        _ => {
                            return Err(JustinoError::RuntimeError {
                                message: "Field name must be a string".to_string(),
                                span: Some(span),
                            });
                        }
                    };

                    let obj = self.get_reg(frame_idx, obj_reg, span)?;
                    let field_result = match obj {
                        Value::StructInstance { fields, .. } | Value::Object(fields) => {
                            fields.borrow().get(&field_name).cloned().unwrap_or(Value::Null)
                        }
                        other => {
                            return Err(JustinoError::RuntimeError {
                                message: format!("Cannot get field on type '{}'", other.type_name()),
                                span: Some(span),
                            });
                        }
                    };

                    self.set_reg(frame_idx, dst, field_result, span)?;
                }
                Opcode::ConcatStrings {
                    dst,
                    start_reg,
                    count,
                } => {
                    let mut result_str = String::new();
                    for i in 0..count {
                        let r = start_reg + i;
                        let val = self.get_reg(frame_idx, r, span)?;
                        result_str.push_str(&val.to_string());
                    }
                    let res_val = self.gc.alloc_string(result_str);
                    self.set_reg(frame_idx, dst, res_val, span)?;
                }
            }
        }

        Ok(Value::Null)
    }

    fn get_reg(&self, frame_idx: usize, reg: u8, span: Span) -> Result<&Value, JustinoError> {
        self.frames[frame_idx]
            .registers
            .get(reg as usize)
            .ok_or_else(|| JustinoError::RuntimeError {
                message: format!("Register R{} out of bounds", reg),
                span: Some(span),
            })
    }

    fn set_reg(&mut self, frame_idx: usize, reg: u8, val: Value, _span: Span) -> Result<(), JustinoError> {
        let frame = &mut self.frames[frame_idx];
        let reg_idx = reg as usize;
        if reg_idx >= frame.registers.len() {
            frame.registers.resize(reg_idx + 1, Value::Null);
        }
        frame.registers[reg_idx] = val;
        Ok(())
    }
}
