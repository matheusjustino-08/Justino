//! Pratt Parser implementation for the Justino programming language.

use crate::error::JustinoError;
use crate::lexer::token::{Token, TokenKind};
use crate::parser::ast::*;
use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest = 0,
    Assignment = 1, // =
    Or = 2,         // ||
    And = 3,        // &&
    Equals = 4,     // ==, !=
    Comparison = 5, // <, >, <=, >=
    Sum = 6,        // +, -
    Product = 7,    // *, /, %
    Unary = 8,      // !, -
    Call = 9,       // () or .field
}

impl Precedence {
    pub fn of_token(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Or => Precedence::Or,
            TokenKind::And => Precedence::And,
            TokenKind::Equal | TokenKind::NotEqual => Precedence::Equals,
            TokenKind::Less | TokenKind::Greater | TokenKind::LessEqual | TokenKind::GreaterEqual => Precedence::Comparison,
            TokenKind::Plus | TokenKind::Minus => Precedence::Sum,
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Precedence::Product,
            TokenKind::LeftParen | TokenKind::Dot => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

/// Recursive Pratt Parser.
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    pub file_id: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, file_id: usize) -> Self {
        Self {
            tokens,
            cursor: 0,
            file_id,
        }
    }

    /// Parses a complete program (.jucode source file).
    pub fn parse_program(&mut self) -> Result<Program, JustinoError> {
        let start_span = self.current_span();
        let mut stmts = Vec::new();

        while !self.is_at_end() {
            if self.check(&TokenKind::Eof) {
                break;
            }
            let stmt = self.parse_statement()?;
            stmts.push(stmt);
        }

        let end_span = self.previous_span();
        let program_span = start_span.merge(&end_span);

        Ok(Program {
            stmts,
            span: program_span,
        })
    }

    /// Parses a single statement.
    pub fn parse_statement(&mut self) -> Result<Stmt, JustinoError> {
        if self.match_token(&TokenKind::Let) {
            self.parse_let_statement()
        } else if self.match_token(&TokenKind::Fn) {
            self.parse_function_statement(false)
        } else if self.check(&TokenKind::Async) && self.check_next(&TokenKind::Fn) {
            self.advance(); // consume async
            self.advance(); // consume fn
            self.parse_function_statement(true)
        } else if self.match_token(&TokenKind::Struct) {
            self.parse_struct_statement()
        } else if self.match_token(&TokenKind::If) {
            self.parse_if_statement()
        } else if self.match_token(&TokenKind::While) {
            self.parse_while_statement()
        } else if self.match_token(&TokenKind::Return) {
            self.parse_return_statement()
        } else if self.check(&TokenKind::LeftBrace) {
            let block = self.parse_block_statement()?;
            Ok(Stmt::Block(block))
        } else {
            self.parse_expr_or_assignment_statement()
        }
    }

    fn parse_let_statement(&mut self) -> Result<Stmt, JustinoError> {
        let let_span = self.previous_span();
        let is_mutable = self.match_token(&TokenKind::Mut);

        let name_token = self.consume_identifier("Expected variable name after 'let'")?;
        let name = match name_token.kind {
            TokenKind::Identifier(n) => n,
            _ => unreachable!(),
        };

        let type_annotation = if self.match_token(&TokenKind::Colon) {
            let type_tok = self.consume_identifier("Expected type name after ':'")?;
            let type_name = match type_tok.kind {
                TokenKind::Identifier(t) => t,
                _ => unreachable!(),
            };
            Some(TypeAnnotation {
                name: type_name,
                span: type_tok.span,
            })
        } else {
            None
        };

        self.consume(&TokenKind::Assign, "Expected '=' in let binding")?;
        let initializer = self.parse_expression(Precedence::Lowest)?;

        let end_span = initializer.span();
        self.match_token(&TokenKind::Semicolon); // optional trailing semicolon

        Ok(Stmt::Let {
            name,
            is_mutable,
            type_annotation,
            initializer,
            span: let_span.merge(&end_span),
        })
    }

    fn parse_function_statement(&mut self, is_async: bool) -> Result<Stmt, JustinoError> {
        let fn_span = self.previous_span();
        let name_tok = self.consume_identifier("Expected function name after 'fn'")?;
        let name = match name_tok.kind {
            TokenKind::Identifier(n) => n,
            _ => unreachable!(),
        };

        self.consume(&TokenKind::LeftParen, "Expected '(' after function name")?;
        let mut params = Vec::new();

        if !self.check(&TokenKind::RightParen) {
            loop {
                let param_span_start = self.current_span();
                let param_tok = self.consume_identifier("Expected parameter name")?;
                let param_name = match param_tok.kind {
                    TokenKind::Identifier(p) => p,
                    _ => unreachable!(),
                };

                let type_annotation = if self.match_token(&TokenKind::Colon) {
                    let t_tok = self.consume_identifier("Expected type name after ':'")?;
                    let t_name = match t_tok.kind {
                        TokenKind::Identifier(t) => t,
                        _ => unreachable!(),
                    };
                    Some(TypeAnnotation {
                        name: t_name,
                        span: t_tok.span,
                    })
                } else {
                    None
                };

                let param_span = param_span_start.merge(&self.previous_span());
                params.push(Param {
                    name: param_name,
                    type_annotation,
                    span: param_span,
                });

                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(&TokenKind::RightParen, "Expected ')' after parameter list")?;

        let return_type = if self.match_token(&TokenKind::Arrow) {
            let ret_tok = self.consume_identifier("Expected return type after '->'")?;
            let ret_name = match ret_tok.kind {
                TokenKind::Identifier(r) => r,
                _ => unreachable!(),
            };
            Some(TypeAnnotation {
                name: ret_name,
                span: ret_tok.span,
            })
        } else {
            None
        };

        let body = self.parse_block_statement()?;
        let total_span = fn_span.merge(&body.span);

        Ok(Stmt::FunctionDef {
            name,
            params,
            return_type,
            body,
            is_async,
            span: total_span,
        })
    }

    fn parse_struct_statement(&mut self) -> Result<Stmt, JustinoError> {
        let struct_span = self.previous_span();
        let name_tok = self.consume_identifier("Expected struct name after 'struct'")?;
        let name = match name_tok.kind {
            TokenKind::Identifier(n) => n,
            _ => unreachable!(),
        };

        self.consume(&TokenKind::LeftBrace, "Expected '{' to start struct body")?;
        let mut fields = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            let field_start = self.current_span();
            let f_tok = self.consume_identifier("Expected field name")?;
            let field_name = match f_tok.kind {
                TokenKind::Identifier(f) => f,
                _ => unreachable!(),
            };

            let type_annotation = if self.match_token(&TokenKind::Colon) {
                let t_tok = self.consume_identifier("Expected field type after ':'")?;
                let t_name = match t_tok.kind {
                    TokenKind::Identifier(t) => t,
                    _ => unreachable!(),
                };
                Some(TypeAnnotation {
                    name: t_name,
                    span: t_tok.span,
                })
            } else {
                None
            };

            let field_span = field_start.merge(&self.previous_span());
            fields.push(Field {
                name: field_name,
                type_annotation,
                span: field_span,
            });

            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }

        self.consume(&TokenKind::RightBrace, "Expected '}' after struct body")?;
        let end_span = self.previous_span();

        Ok(Stmt::StructDef {
            name,
            fields,
            span: struct_span.merge(&end_span),
        })
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, JustinoError> {
        let if_span = self.previous_span();
        let condition = self.parse_expression(Precedence::Lowest)?;
        let then_branch = self.parse_block_statement()?;

        let else_branch = if self.match_token(&TokenKind::Else) {
            if self.match_token(&TokenKind::If) {
                let nested_if = self.parse_if_statement()?;
                let span = nested_if.span();
                Some(BlockStmt {
                    stmts: vec![nested_if],
                    span,
                })
            } else {
                Some(self.parse_block_statement()?)
            }
        } else {
            None
        };

        let end_span = else_branch.as_ref().map(|b| b.span).unwrap_or(then_branch.span);

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
            span: if_span.merge(&end_span),
        })
    }

    fn parse_while_statement(&mut self) -> Result<Stmt, JustinoError> {
        let while_span = self.previous_span();
        let condition = self.parse_expression(Precedence::Lowest)?;
        let body = self.parse_block_statement()?;
        let end_span = body.span;

        Ok(Stmt::While {
            condition,
            body,
            span: while_span.merge(&end_span),
        })
    }

    fn parse_return_statement(&mut self) -> Result<Stmt, JustinoError> {
        let ret_span = self.previous_span();
        let value = if self.check(&TokenKind::Semicolon) || self.check(&TokenKind::RightBrace) || self.is_at_end() {
            None
        } else {
            Some(self.parse_expression(Precedence::Lowest)?)
        };

        let end_span = value.as_ref().map(|e| e.span()).unwrap_or(ret_span);
        self.match_token(&TokenKind::Semicolon);

        Ok(Stmt::Return {
            value,
            span: ret_span.merge(&end_span),
        })
    }

    fn parse_block_statement(&mut self) -> Result<BlockStmt, JustinoError> {
        let start_span = self.current_span();
        self.consume(&TokenKind::LeftBrace, "Expected '{' to begin block")?;
        let mut stmts = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            stmts.push(self.parse_statement()?);
        }

        self.consume(&TokenKind::RightBrace, "Expected '}' to end block")?;
        let end_span = self.previous_span();

        Ok(BlockStmt {
            stmts,
            span: start_span.merge(&end_span),
        })
    }

    fn parse_expr_or_assignment_statement(&mut self) -> Result<Stmt, JustinoError> {
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.match_token(&TokenKind::Assign) {
            let val = self.parse_expression(Precedence::Lowest)?;
            let total_span = expr.span().merge(&val.span());
            self.match_token(&TokenKind::Semicolon);
            Ok(Stmt::Assignment {
                target: expr,
                value: val,
                span: total_span,
            })
        } else {
            self.match_token(&TokenKind::Semicolon);
            Ok(Stmt::Expr(expr))
        }
    }

    /// Primary Pratt Expression parsing loop.
    pub fn parse_expression(&mut self, precedence: Precedence) -> Result<Expr, JustinoError> {
        let mut left = self.parse_prefix()?;

        while !self.is_at_end() {
            let next_prec = Precedence::of_token(&self.peek().kind);
            if precedence >= next_prec {
                break;
            }

            left = self.parse_infix(left)?;
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expr, JustinoError> {
        let tok = self.advance();
        let span = tok.span;

        match tok.kind {
            TokenKind::Int(val) => Ok(Expr::Literal(Literal::Int(val), span)),
            TokenKind::Float(val) => Ok(Expr::Literal(Literal::Float(val), span)),
            TokenKind::String(val) => Ok(Expr::Literal(Literal::String(val), span)),
            TokenKind::True => Ok(Expr::Literal(Literal::Bool(true), span)),
            TokenKind::False => Ok(Expr::Literal(Literal::Bool(false), span)),
            TokenKind::Null => Ok(Expr::Literal(Literal::Null, span)),
            TokenKind::Identifier(name) => {
                // Check if this identifier is a struct initialization: `Point { x: 10, y: 20 }`
                if self.check(&TokenKind::LeftBrace) && self.looks_like_struct_init() {
                    self.consume(&TokenKind::LeftBrace, "Expected '{'")?;
                    let mut fields = Vec::new();
                    while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                        let f_tok = self.consume_identifier("Expected field name in struct initializer")?;
                        let f_name = match f_tok.kind {
                            TokenKind::Identifier(f) => f,
                            _ => unreachable!(),
                        };
                        self.consume(&TokenKind::Colon, "Expected ':' after field name")?;
                        let f_val = self.parse_expression(Precedence::Lowest)?;
                        fields.push((f_name, f_val));

                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.consume(&TokenKind::RightBrace, "Expected '}'")?;
                    let end_span = self.previous_span();
                    Ok(Expr::StructInit {
                        name,
                        fields,
                        span: span.merge(&end_span),
                    })
                } else {
                    Ok(Expr::Variable(name, span))
                }
            }
            TokenKind::Minus => {
                let operand = self.parse_expression(Precedence::Unary)?;
                let total_span = span.merge(&operand.span());
                Ok(Expr::Unary {
                    op: UnaryOp::Negate,
                    operand: Box::new(operand),
                    span: total_span,
                })
            }
            TokenKind::Not => {
                let operand = self.parse_expression(Precedence::Unary)?;
                let total_span = span.merge(&operand.span());
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(operand),
                    span: total_span,
                })
            }
            TokenKind::LeftParen => {
                let expr = self.parse_expression(Precedence::Lowest)?;
                self.consume(&TokenKind::RightParen, "Expected ')' after group expression")?;
                Ok(expr)
            }
            TokenKind::Spawn => {
                let expr = self.parse_expression(Precedence::Unary)?;
                let total_span = span.merge(&expr.span());
                Ok(Expr::Spawn {
                    expr: Box::new(expr),
                    span: total_span,
                })
            }
            TokenKind::Await => {
                let expr = self.parse_expression(Precedence::Unary)?;
                let total_span = span.merge(&expr.span());
                Ok(Expr::Await {
                    expr: Box::new(expr),
                    span: total_span,
                })
            }
            TokenKind::StringSegment(initial_seg) => {
                let mut parts = vec![Expr::Literal(Literal::String(initial_seg), span)];
                
                // If followed by `${`, continue parsing interpolated string parts
                while self.match_token(&TokenKind::DollarBrace) {
                    let interp_expr = self.parse_expression(Precedence::Lowest)?;
                    parts.push(interp_expr);
                    self.consume(&TokenKind::RightBrace, "Expected '}' to close string interpolation '${...}'")?;

                    if let TokenKind::StringSegment(seg) = &self.peek().kind {
                        let seg_span = self.peek().span;
                        let seg_val = seg.clone();
                        self.advance();
                        parts.push(Expr::Literal(Literal::String(seg_val), seg_span));
                    }
                }

                let total_span = parts.last().map(|p| span.merge(&p.span())).unwrap_or(span);
                Ok(Expr::InterpolatedString {
                    parts,
                    span: total_span,
                })
            }
            other => Err(JustinoError::ParseError {
                message: format!("Unexpected token '{:?}' in expression prefix", other),
                span,
            }),
        }
    }

    fn parse_infix(&mut self, left: Expr) -> Result<Expr, JustinoError> {
        let tok = self.advance();
        let span = tok.span;

        match tok.kind {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Percent
            | TokenKind::Equal
            | TokenKind::NotEqual
            | TokenKind::Less
            | TokenKind::Greater
            | TokenKind::LessEqual
            | TokenKind::GreaterEqual
            | TokenKind::And
            | TokenKind::Or => {
                let prec = Precedence::of_token(&tok.kind);
                let right = self.parse_expression(prec)?;
                let total_span = left.span().merge(&right.span());

                let op = match tok.kind {
                    TokenKind::Plus => BinaryOp::Add,
                    TokenKind::Minus => BinaryOp::Sub,
                    TokenKind::Star => BinaryOp::Mul,
                    TokenKind::Slash => BinaryOp::Div,
                    TokenKind::Percent => BinaryOp::Mod,
                    TokenKind::Equal => BinaryOp::Equal,
                    TokenKind::NotEqual => BinaryOp::NotEqual,
                    TokenKind::Less => BinaryOp::Less,
                    TokenKind::Greater => BinaryOp::Greater,
                    TokenKind::LessEqual => BinaryOp::LessEqual,
                    TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
                    TokenKind::And => BinaryOp::And,
                    TokenKind::Or => BinaryOp::Or,
                    _ => unreachable!(),
                };

                Ok(Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                    span: total_span,
                })
            }
            TokenKind::LeftParen => {
                let mut args = Vec::new();
                if !self.check(&TokenKind::RightParen) {
                    loop {
                        args.push(self.parse_expression(Precedence::Lowest)?);
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.consume(&TokenKind::RightParen, "Expected ')' after function arguments")?;
                let end_span = self.previous_span();
                let total_span = left.span().merge(&end_span);

                Ok(Expr::Call {
                    callee: Box::new(left),
                    args,
                    span: total_span,
                })
            }
            TokenKind::Dot => {
                let member_tok = self.consume_identifier("Expected field or method name after '.'")?;
                let member = match member_tok.kind {
                    TokenKind::Identifier(m) => m,
                    _ => unreachable!(),
                };
                let total_span = left.span().merge(&member_tok.span);

                Ok(Expr::MemberAccess {
                    object: Box::new(left),
                    member,
                    span: total_span,
                })
            }
            other => Err(JustinoError::ParseError {
                message: format!("Unexpected token '{:?}' in expression infix", other),
                span,
            }),
        }
    }

    fn looks_like_struct_init(&self) -> bool {
        // Lookahead past LeftBrace: identifier followed by ':'
        if self.cursor + 1 < self.tokens.len() {
            if matches!(self.tokens[self.cursor].kind, TokenKind::LeftBrace) {
                if let TokenKind::Identifier(_) = &self.tokens[self.cursor + 1].kind {
                    if self.cursor + 2 < self.tokens.len() {
                        return matches!(self.tokens[self.cursor + 2].kind, TokenKind::Colon);
                    }
                }
            }
        }
        false
    }

    fn consume_identifier(&mut self, error_msg: &str) -> Result<Token, JustinoError> {
        if let TokenKind::Identifier(_) = self.peek().kind {
            Ok(self.advance())
        } else {
            Err(JustinoError::ParseError {
                message: error_msg.to_string(),
                span: self.peek().span,
            })
        }
    }

    fn consume(&mut self, expected: &TokenKind, error_msg: &str) -> Result<Token, JustinoError> {
        if self.check(expected) {
            Ok(self.advance())
        } else {
            Err(JustinoError::ParseError {
                message: format!("{}, found '{:?}'", error_msg, self.peek().kind),
                span: self.peek().span,
            })
        }
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
        }
    }

    fn check_next(&self, kind: &TokenKind) -> bool {
        if self.cursor + 1 >= self.tokens.len() {
            false
        } else {
            std::mem::discriminant(&self.tokens[self.cursor + 1].kind) == std::mem::discriminant(kind)
        }
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.cursor += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn peek(&self) -> &Token {
        static DUMMY_EOF: Token = Token {
            kind: TokenKind::Eof,
            span: Span::dummy(),
        };
        self.tokens.get(self.cursor).unwrap_or_else(|| {
            self.tokens.last().unwrap_or(&DUMMY_EOF)
        })
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.cursor - 1).cloned().unwrap_or_else(|| {
            Token::new(TokenKind::Eof, Span::dummy())
        })
    }

    fn current_span(&self) -> Span {
        self.peek().span
    }

    fn previous_span(&self) -> Span {
        self.previous().span
    }
}
