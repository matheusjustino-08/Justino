//! Abstract Syntax Tree (AST) node definitions for the Justino language.

use crate::span::Span;

/// Top-level Program container.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// Function parameter definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
    pub span: Span,
}

/// Struct field definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub type_annotation: Option<TypeAnnotation>,
    pub span: Span,
}

/// Optional type annotation node.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation {
    pub name: String,
    pub span: Span,
}

/// Block statement containing zero or more statements.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// AST Statement nodes.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// Variable binding: `let [mut] name [: Type] = initializer;`
    Let {
        name: String,
        is_mutable: bool,
        type_annotation: Option<TypeAnnotation>,
        initializer: Expr,
        span: Span,
    },
    /// Re-assignment: `target = value;`
    Assignment {
        target: Expr,
        value: Expr,
        span: Span,
    },
    /// Function declaration: `[async] fn name(params) [-> Type] { body }`
    FunctionDef {
        name: String,
        params: Vec<Param>,
        return_type: Option<TypeAnnotation>,
        body: BlockStmt,
        is_async: bool,
        span: Span,
    },
    /// Struct definition: `struct Name { field1: Type1, ... }`
    StructDef {
        name: String,
        fields: Vec<Field>,
        span: Span,
    },
    /// Conditional execution: `if condition { ... } else { ... }`
    If {
        condition: Expr,
        then_branch: BlockStmt,
        else_branch: Option<BlockStmt>,
        span: Span,
    },
    /// Loop execution: `while condition { ... }`
    While {
        condition: Expr,
        body: BlockStmt,
        span: Span,
    },
    /// Return statement: `return [expr];`
    Return {
        value: Option<Expr>,
        span: Span,
    },
    /// Expression statement: `expr;`
    Expr(Expr),
    /// Nested block statement: `{ ... }`
    Block(BlockStmt),
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Let { span, .. } => *span,
            Stmt::Assignment { span, .. } => *span,
            Stmt::FunctionDef { span, .. } => *span,
            Stmt::StructDef { span, .. } => *span,
            Stmt::If { span, .. } => *span,
            Stmt::While { span, .. } => *span,
            Stmt::Return { span, .. } => *span,
            Stmt::Expr(expr) => expr.span(),
            Stmt::Block(block) => block.span,
        }
    }
}

/// Literal scalar values.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Negate,
    Not,
}

/// AST Expression nodes.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal, Span),
    Variable(String, Span),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
        span: Span,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    StructInit {
        name: String,
        fields: Vec<(String, Expr)>,
        span: Span,
    },
    MemberAccess {
        object: Box<Expr>,
        member: String,
        span: Span,
    },
    Spawn {
        expr: Box<Expr>,
        span: Span,
    },
    Await {
        expr: Box<Expr>,
        span: Span,
    },
    InterpolatedString {
        parts: Vec<Expr>,
        span: Span,
    },
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(_, span) => *span,
            Expr::Variable(_, span) => *span,
            Expr::Binary { span, .. } => *span,
            Expr::Unary { span, .. } => *span,
            Expr::Call { span, .. } => *span,
            Expr::StructInit { span, .. } => *span,
            Expr::MemberAccess { span, .. } => *span,
            Expr::Spawn { span, .. } => *span,
            Expr::Await { span, .. } => *span,
            Expr::InterpolatedString { span, .. } => *span,
        }
    }
}
