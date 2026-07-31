//! Register-based Opcode instructions and bytecode chunk representation.

use crate::span::Span;
use crate::vm::value::Value;

/// Register-based Opcodes for the Justino Virtual Machine.
#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    /// Load constant from table into register `dst`
    LoadConst { dst: u8, const_idx: u16 },
    /// Move value from `src` to `dst`
    Move { dst: u8, src: u8 },

    // --- Arithmetic ---
    Add { dst: u8, lhs: u8, rhs: u8 },
    Sub { dst: u8, lhs: u8, rhs: u8 },
    Mul { dst: u8, lhs: u8, rhs: u8 },
    Div { dst: u8, lhs: u8, rhs: u8 },
    Mod { dst: u8, lhs: u8, rhs: u8 },
    Neg { dst: u8, src: u8 },
    Not { dst: u8, src: u8 },

    // --- Comparison & Logical ---
    Equal { dst: u8, lhs: u8, rhs: u8 },
    NotEqual { dst: u8, lhs: u8, rhs: u8 },
    LessThan { dst: u8, lhs: u8, rhs: u8 },
    GreaterThan { dst: u8, lhs: u8, rhs: u8 },
    LessEqual { dst: u8, lhs: u8, rhs: u8 },
    GreaterEqual { dst: u8, lhs: u8, rhs: u8 },

    // --- Jumps ---
    Jump { offset: i16 },
    JumpIfFalse { condition: u8, offset: i16 },

    // --- Functions & Concurrency ---
    Call { dst: u8, func_reg: u8, arg_start: u8, arg_count: u8 },
    Return { src: Option<u8> },
    Spawn { func_reg: u8 },

    // --- Struct & Field Operations ---
    NewStruct { dst: u8, name_idx: u16, field_count: u8 },
    SetField { obj_reg: u8, field_idx: u16, val_reg: u8 },
    GetField { dst: u8, obj_reg: u8, field_idx: u16 },

    // --- Helpers ---
    LoadNull { dst: u8 },
    LoadBool { dst: u8, val: bool },
    ConcatStrings { dst: u8, start_reg: u8, count: u8 },
}

/// Represents a compiled function or top-level bytecode chunk.
#[derive(Debug, Clone, PartialEq)]
pub struct CompiledFunction {
    pub name: String,
    pub arity: u8,
    pub num_registers: u8,
    pub instructions: Vec<Opcode>,
    pub constants: Vec<Value>,
    pub spans: Vec<Span>,
}

impl CompiledFunction {
    pub fn new(name: impl Into<String>, arity: u8) -> Self {
        Self {
            name: name.into(),
            arity,
            num_registers: arity,
            instructions: Vec::new(),
            constants: Vec::new(),
            spans: Vec::new(),
        }
    }

    /// Appends an instruction and its associated source span.
    pub fn emit(&mut self, op: Opcode, span: Span) -> usize {
        let idx = self.instructions.len();
        self.instructions.push(op);
        self.spans.push(span);
        idx
    }

    /// Adds a constant value to the constant pool and returns its index.
    pub fn add_constant(&mut self, val: Value) -> u16 {
        for (idx, existing) in self.constants.iter().enumerate() {
            if existing == &val {
                return idx as u16;
            }
        }
        let idx = self.constants.len() as u16;
        self.constants.push(val);
        idx
    }
}
