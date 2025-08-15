//! 语法分析器模块
//!
//! 将词法分析器生成的Token流转换为抽象语法树(AST)
//! 实现递归下降解析算法和Pratt解析算法处理运算符优先级

use crate::data::{Locatable, Token};
use std::fmt;

/// 解析错误类型
#[derive(Debug, PartialEq)]
pub enum ParseError {
    /// 意外的Token
    UnexpectedToken(String),
    /// 缺少必需的Token
    MissingToken(String),
    /// 无效的表达式
    InvalidExpression,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            ParseError::MissingToken(token) => write!(f, "Missing token: {}", token),
            ParseError::InvalidExpression => write!(f, "Invalid expression"),
        }
    }
}

/// AST节点基本特质
pub trait AstNode {
    /// 获取节点在源代码中的位置信息
    fn location(&self) -> &crate::data::Location;
}

/// 字面量表达式
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(String),
    Float(String),
    String(String),
    Char(char),
    Boolean(bool),
}

/// 二元操作符
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    // 算术操作符
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // 比较操作符
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // 逻辑操作符
    And, // &&
    Or,  // ||
}

/// 一元操作符
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate, // -
    Not,    // !
}

/// 效果声明中的操作符
#[derive(Debug, Clone, PartialEq)]
pub struct EffectOperation {
    pub name: String,
    pub parameters: Vec<(String, String)>, // (参数名, 类型)
    pub return_type: Option<String>,
}

/// 效果声明
#[derive(Debug, Clone, PartialEq)]
pub struct EffectDeclaration {
    pub name: String,
    pub operations: Vec<EffectOperation>,
}

/// 处理器子句
#[derive(Debug, Clone, PartialEq)]
pub struct HandlerClause {
    pub operation: String,
    pub parameters: Vec<String>,
    pub body: Vec<Stmt>,
}

/// 处理器声明
#[derive(Debug, Clone, PartialEq)]
pub struct HandlerDeclaration {
    pub effect: String,
    pub clauses: Vec<HandlerClause>,
}

/// 效果组声明
#[derive(Debug, Clone, PartialEq)]
pub struct EffectGroupDeclaration {
    pub name: String,
    pub effects: Vec<String>,
}

/// 处理器组声明
#[derive(Debug, Clone, PartialEq)]
pub struct HandlerGroupDeclaration {
    pub name: String,
    pub handlers: Vec<String>,
}

/// 表达式节点
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// 字面量
    Literal {
        location_line: usize,
        location_column: usize,
        location_file: String,
        value: Literal,
    },

    /// 标识符
    Identifier {
        location_line: usize,
        location_column: usize,
        location_file: String,
        name: String,
    },

    /// 二元表达式
    Binary {
        location_line: usize,
        location_column: usize,
        location_file: String,
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },

    /// 一元表达式
    Unary {
        location_line: usize,
        location_column: usize,
        location_file: String,
        operator: UnaryOperator,
        operand: Box<Expr>,
    },

    /// 函数调用
    Call {
        location_line: usize,
        location_column: usize,
        location_file: String,
        function: Box<Expr>,
        arguments: Vec<Expr>,
    },

    /// 分组表达式 (...)
    Grouping {
        location_line: usize,
        location_column: usize,
        location_file: String,
        expression: Box<Expr>,
    },

    /// 效果操作调用
    EffectOperation {
        location_line: usize,
        location_column: usize,
        location_file: String,
        effect: String,
        operation: String,
        arguments: Vec<Expr>,
    },
}

impl Expr {
    pub fn location(&self) -> crate::data::Location {
        match self {
            Expr::Literal {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Expr::Identifier {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Expr::Binary {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Expr::Unary {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Expr::Call {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Expr::Grouping {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Expr::EffectOperation {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
        }
    }
}

/// 语句节点
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// 表达式语句
    Expression {
        location_line: usize,
        location_column: usize,
        location_file: String,
        expression: Expr,
    },

    /// let声明语句
    Let {
        location_line: usize,
        location_column: usize,
        location_file: String,
        identifier: String,
        initializer: Option<Expr>,
    },

    /// var声明语句
    Var {
        location_line: usize,
        location_column: usize,
        location_file: String,
        identifier: String,
        initializer: Option<Expr>,
    },

    /// 函数声明语句
    Function {
        location_line: usize,
        location_column: usize,
        location_file: String,
        name: String,
        parameters: Vec<String>,
        body: Vec<Stmt>,
    },

    /// 块语句
    Block {
        location_line: usize,
        location_column: usize,
        location_file: String,
        statements: Vec<Stmt>,
    },

    /// 效果声明语句
    Effect {
        location_line: usize,
        location_column: usize,
        location_file: String,
        declaration: EffectDeclaration,
    },

    /// 处理器声明语句
    Handler {
        location_line: usize,
        location_column: usize,
        location_file: String,
        declaration: HandlerDeclaration,
    },

    /// 效果组声明语句
    EffectGroup {
        location_line: usize,
        location_column: usize,
        location_file: String,
        declaration: EffectGroupDeclaration,
    },

    /// 处理器组声明语句
    HandlerGroup {
        location_line: usize,
        location_column: usize,
        location_file: String,
        declaration: HandlerGroupDeclaration,
    },
}

impl Stmt {
    pub fn location(&self) -> crate::data::Location {
        match self {
            Stmt::Expression {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Stmt::Let {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Stmt::Var {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Stmt::Function {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Stmt::Block {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Stmt::Effect {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Stmt::Handler {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Stmt::EffectGroup {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
            Stmt::HandlerGroup {
                location_line,
                location_column,
                location_file,
                ..
            } => crate::data::Location {
                line: *location_line,
                column: *location_column,
                file: location_file.as_str(),
            },
        }
    }
}

/// 语法分析器
pub struct Parser<'a> {
    tokens: Vec<Locatable<'a, Token>>,
    current: usize,
}

impl<'a> Parser<'a> {
    /// 创建新的语法分析器实例
    pub fn new(tokens: Vec<Locatable<'a, Token>>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// 解析入口点 - 解析整个程序
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_declaration()?);
        }

        Ok(statements)
    }

    /// 解析声明
    fn parse_declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[
            Token::Let,
            Token::Var,
            Token::Fn,
            Token::Effect,
            Token::Handle,
        ]) {
            // 回退一个token以便在具体解析函数中处理
            self.current -= 1;
            self.parse_declaration_statement()
        } else if let Token::Identifier(ref ident) = self.peek().data {
            // 检查是否是effect_group或handler_group关键字
            if ident == "effect_group" || ident == "handler_group" {
                self.parse_declaration_statement()
            } else {
                self.parse_statement()
            }
        } else {
            self.parse_statement()
        }
    }

    /// 解析声明语句
    fn parse_declaration_statement(&mut self) -> Result<Stmt, ParseError> {
        // 根据当前token类型决定解析哪种声明
        match self.peek().data {
            Token::Let => self.parse_let_declaration(),
            Token::Var => self.parse_var_declaration(),
            Token::Fn => self.parse_function_declaration(),
            Token::Effect => self.parse_effect_declaration(),
            Token::Handle => self.parse_handler_declaration(),
            Token::Identifier(ref ident) if ident == "effect_group" => {
                self.parse_effect_group_declaration()
            }
            Token::Identifier(ref ident) if ident == "handler_group" => {
                self.parse_handler_group_declaration()
            }
            _ => Err(ParseError::UnexpectedToken(format!(
                "{:?}",
                self.peek().data
            ))),
        }
    }

    /// 解析let声明
    fn parse_let_declaration(&mut self) -> Result<Stmt, ParseError> {
        let token = self.consume(&Token::Let, "Expected 'let' keyword")?;
        let location_line = token.location.line;
        let location_column = token.location.column;
        let location_file = token.location.file.to_string();

        let identifier = if let Token::Identifier(name) = &self.peek().data {
            name.clone()
        } else {
            return Err(ParseError::UnexpectedToken(
                "Expected identifier".to_string(),
            ));
        };

        self.advance(); // 消费标识符

        let initializer = if self.match_token(&[Token::Equal]) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(&Token::Semicolon, "Expected ';' after let declaration")?;

        Ok(Stmt::Let {
            location_line,
            location_column,
            location_file,
            identifier,
            initializer,
        })
    }

    /// 解析var声明
    fn parse_var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let token = self.consume(&Token::Var, "Expected 'var' keyword")?;
        let location_line = token.location.line;
        let location_column = token.location.column;
        let location_file = token.location.file.to_string();

        let identifier = if let Token::Identifier(name) = &self.peek().data {
            name.clone()
        } else {
            return Err(ParseError::UnexpectedToken(
                "Expected identifier".to_string(),
            ));
        };

        self.advance(); // 消费标识符

        let initializer = if self.match_token(&[Token::Equal]) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume(&Token::Semicolon, "Expected ';' after var declaration")?;

        Ok(Stmt::Var {
            location_line,
            location_column,
            location_file,
            identifier,
            initializer,
        })
    }

    /// 解析函数声明
    fn parse_function_declaration(&mut self) -> Result<Stmt, ParseError> {
        let token = self.consume(&Token::Fn, "Expected 'fn' keyword")?;
        let location_line = token.location.line;
        let location_column = token.location.column;
        let location_file = token.location.file.to_string();

        let name = if let Token::Identifier(name) = &self.peek().data {
            name.clone()
        } else {
            return Err(ParseError::UnexpectedToken(
                "Expected function name".to_string(),
            ));
        };

        self.advance(); // 消费函数名

        self.consume(&Token::LParen, "Expected '(' after function name")?;

        let mut parameters = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                if let Token::Identifier(name) = &self.peek().data {
                    parameters.push(name.clone());
                    self.advance();
                } else {
                    return Err(ParseError::UnexpectedToken(
                        "Expected parameter name".to_string(),
                    ));
                }

                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }

        self.consume(&Token::RParen, "Expected ')' after parameters")?;

        // 检查是否有返回类型
        if self.match_token(&[Token::Arrow]) {
            // 解析返回类型（这里我们只是消费它，因为我们还没有实现完整的类型系统）
            if let Token::Identifier(_) = &self.peek().data {
                self.advance(); // 消费返回类型
            } else {
                return Err(ParseError::UnexpectedToken(
                    "Expected return type".to_string(),
                ));
            }
        }

        // 检查是否有effects关键字
        let mut effect_list = Vec::new();
        if let Token::Identifier(ref ident) = self.peek().data
            && ident == "effects"
        {
            self.advance(); // 消费effects关键字

            // 解析效果列表
            loop {
                if let Token::Identifier(effect_name) = &self.peek().data {
                    effect_list.push(effect_name.clone());
                    self.advance(); // 消费效果名
                } else {
                    return Err(ParseError::UnexpectedToken(
                        "Expected effect name".to_string(),
                    ));
                }

                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }

        self.consume(&Token::LBrace, "Expected '{' before function body")?;

        let body = self.parse_block_statement()?;
        if let Stmt::Block { statements, .. } = body {
            // 注意：目前我们没有在AST中存储effect_list和返回类型，因为我们还没有实现完整的类型系统
            // 这将在后续阶段实现
            Ok(Stmt::Function {
                location_line,
                location_column,
                location_file,
                name,
                parameters,
                body: statements,
            })
        } else {
            // 这不应该发生，因为parse_block_statement总是返回Block语句
            Err(ParseError::InvalidExpression)
        }
    }

    /// 解析效果声明
    fn parse_effect_declaration(&mut self) -> Result<Stmt, ParseError> {
        let token = self.consume(&Token::Effect, "Expected 'effect' keyword")?;
        let location_line = token.location.line;
        let location_column = token.location.column;
        let location_file = token.location.file.to_string();

        let name = if let Token::Identifier(name) = &self.peek().data {
            name.clone()
        } else {
            return Err(ParseError::UnexpectedToken(
                "Expected effect name".to_string(),
            ));
        };

        self.advance(); // 消费效果名

        self.consume(&Token::LBrace, "Expected '{' after effect name")?;

        let mut operations = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            self.consume(&Token::Fn, "Expected 'fn' keyword")?;

            let op_name = if let Token::Identifier(name) = &self.peek().data {
                name.clone()
            } else {
                return Err(ParseError::UnexpectedToken(
                    "Expected operation name".to_string(),
                ));
            };

            self.advance(); // 消费操作名

            self.consume(&Token::LParen, "Expected '(' after operation name")?;

            let mut parameters = Vec::new();
            if !self.check(&Token::RParen) {
                loop {
                    let param_name = if let Token::Identifier(name) = &self.peek().data {
                        name.clone()
                    } else {
                        return Err(ParseError::UnexpectedToken(
                            "Expected parameter name".to_string(),
                        ));
                    };

                    self.advance(); // 消费参数名

                    self.consume(&Token::Colon, "Expected ':' after parameter name")?;

                    let param_type = if let Token::Identifier(type_name) = &self.peek().data {
                        type_name.clone()
                    } else {
                        return Err(ParseError::UnexpectedToken(
                            "Expected parameter type".to_string(),
                        ));
                    };

                    self.advance(); // 消费参数类型

                    parameters.push((param_name, param_type));

                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }

            self.consume(&Token::RParen, "Expected ')' after parameters")?;

            let return_type = if self.match_token(&[Token::Arrow]) {
                if let Token::Identifier(type_name) = &self.peek().data {
                    let type_name = type_name.clone();
                    self.advance(); // 消费返回类型
                    Some(type_name)
                } else {
                    return Err(ParseError::UnexpectedToken(
                        "Expected return type".to_string(),
                    ));
                }
            } else {
                None
            };

            self.consume(&Token::Semicolon, "Expected ';' after operation")?;

            operations.push(EffectOperation {
                name: op_name,
                parameters,
                return_type,
            });
        }

        self.consume(&Token::RBrace, "Expected '}' after effect operations")?;

        Ok(Stmt::Effect {
            location_line,
            location_column,
            location_file,
            declaration: EffectDeclaration { name, operations },
        })
    }

    /// 解析处理器声明
    fn parse_handler_declaration(&mut self) -> Result<Stmt, ParseError> {
        let token = self.consume(&Token::Handle, "Expected 'handle' keyword")?;
        let location_line = token.location.line;
        let location_column = token.location.column;
        let location_file = token.location.file.to_string();

        let effect = if let Token::Identifier(name) = &self.peek().data {
            name.clone()
        } else {
            return Err(ParseError::UnexpectedToken(
                "Expected effect name".to_string(),
            ));
        };

        self.advance(); // 消费效果名

        self.consume(&Token::LBrace, "Expected '{' after effect name")?;

        let mut clauses = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            let operation = if let Token::Identifier(name) = &self.peek().data {
                name.clone()
            } else {
                return Err(ParseError::UnexpectedToken(
                    "Expected operation name".to_string(),
                ));
            };

            self.advance(); // 消费操作名

            self.consume(&Token::LParen, "Expected '(' after operation name")?;

            let mut parameters = Vec::new();
            if !self.check(&Token::RParen) {
                loop {
                    let param_name = if let Token::Identifier(name) = &self.peek().data {
                        name.clone()
                    } else {
                        return Err(ParseError::UnexpectedToken(
                            "Expected parameter name".to_string(),
                        ));
                    };

                    self.advance(); // 消费参数名
                    parameters.push(param_name);

                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }

            self.consume(&Token::RParen, "Expected ')' after parameters")?;
            self.consume(&Token::LBrace, "Expected '{' before clause body")?;

            let body = self.parse_block_statement()?;
            if let Stmt::Block { statements, .. } = body {
                clauses.push(HandlerClause {
                    operation,
                    parameters,
                    body: statements,
                });
            } else {
                return Err(ParseError::InvalidExpression);
            }
        }

        self.consume(&Token::RBrace, "Expected '}' after handler clauses")?;

        Ok(Stmt::Handler {
            location_line,
            location_column,
            location_file,
            declaration: HandlerDeclaration { effect, clauses },
        })
    }

    /// 解析效果组声明
    fn parse_effect_group_declaration(&mut self) -> Result<Stmt, ParseError> {
        // 消费effect_group标识符
        let token = self.advance().clone();
        let location_line = token.location.line;
        let location_column = token.location.column;
        let location_file = token.location.file.to_string();

        let name = if let Token::Identifier(name) = &self.peek().data {
            name.clone()
        } else {
            return Err(ParseError::UnexpectedToken(
                "Expected effect group name".to_string(),
            ));
        };

        self.advance(); // 消费组名

        self.consume(&Token::Equal, "Expected '=' after effect group name")?;

        let mut effects = Vec::new();
        loop {
            if let Token::Identifier(effect_name) = &self.peek().data {
                effects.push(effect_name.clone());
                self.advance(); // 消费效果名
            } else {
                return Err(ParseError::UnexpectedToken(
                    "Expected effect name".to_string(),
                ));
            }

            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }

        self.consume(
            &Token::Semicolon,
            "Expected ';' after effect group declaration",
        )?;

        Ok(Stmt::EffectGroup {
            location_line,
            location_column,
            location_file,
            declaration: EffectGroupDeclaration { name, effects },
        })
    }

    /// 解析处理器组声明
    fn parse_handler_group_declaration(&mut self) -> Result<Stmt, ParseError> {
        // 消费handler_group标识符
        let token = self.advance().clone();
        let location_line = token.location.line;
        let location_column = token.location.column;
        let location_file = token.location.file.to_string();

        let name = if let Token::Identifier(name) = &self.peek().data {
            name.clone()
        } else {
            return Err(ParseError::UnexpectedToken(
                "Expected handler group name".to_string(),
            ));
        };

        self.advance(); // 消费组名

        self.consume(&Token::Equal, "Expected '=' after handler group name")?;

        let mut handlers = Vec::new();
        loop {
            if let Token::Identifier(handler_name) = &self.peek().data {
                handlers.push(handler_name.clone());
                self.advance(); // 消费处理器名
            } else {
                return Err(ParseError::UnexpectedToken(
                    "Expected handler name".to_string(),
                ));
            }

            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }

        self.consume(
            &Token::Semicolon,
            "Expected ';' after handler group declaration",
        )?;

        Ok(Stmt::HandlerGroup {
            location_line,
            location_column,
            location_file,
            declaration: HandlerGroupDeclaration { name, handlers },
        })
    }

    /// 解析语句
    fn parse_statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[Token::LBrace]) {
            self.parse_block_statement()
        } else {
            self.parse_expression_statement()
        }
    }

    /// 解析块语句
    fn parse_block_statement(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&Token::RBrace) && !self.is_at_end() {
            // 在块中，我们可以解析声明或语句
            if self.match_token(&[
                Token::Let,
                Token::Var,
                Token::Fn,
                Token::Effect,
                Token::Handle,
            ]) {
                self.current -= 1; // 回退token
                statements.push(self.parse_declaration_statement()?);
            } else if let Token::Identifier(ref ident) = self.peek().data {
                // 检查是否是effect_group或handler_group关键字
                if ident == "effect_group" || ident == "handler_group" {
                    statements.push(self.parse_declaration_statement()?);
                } else {
                    statements.push(self.parse_statement()?);
                }
            } else {
                statements.push(self.parse_statement()?);
            }
        }

        let token = self.consume(&Token::RBrace, "Expected '}' after block")?;
        let location_line = token.location.line;
        let location_column = token.location.column;
        let location_file = token.location.file.to_string();

        Ok(Stmt::Block {
            location_line,
            location_column,
            location_file,
            statements,
        })
    }

    /// 解析表达式语句
    fn parse_expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expression = self.parse_expression()?;
        let location = expression.location();
        let location_line = location.line;
        let location_column = location.column;
        let location_file = location.file.to_string();

        self.consume(&Token::Semicolon, "Expected ';' after expression")?;

        Ok(Stmt::Expression {
            location_line,
            location_column,
            location_file,
            expression,
        })
    }

    /// 解析表达式（使用Pratt解析算法）
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_binary_expression(0)
    }

    /// 解析二元表达式（Pratt解析算法核心）
    fn parse_binary_expression(&mut self, precedence: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary_expression()?;

        while let Some(current_precedence) = self.get_precedence() {
            if current_precedence < precedence {
                break;
            }

            let operator = self.parse_binary_operator()?;
            self.advance(); // 消费操作符

            let right = self.parse_binary_expression(current_precedence + 1)?;

            let location = left.location();
            let location_line = location.line;
            let location_column = location.column;
            let location_file = location.file.to_string();

            left = Expr::Binary {
                location_line,
                location_column,
                location_file,
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// 解析一元表达式
    fn parse_unary_expression(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[Token::Minus, Token::Bang]) {
            let operator_token = self.previous().data.clone();
            let operator = match operator_token {
                Token::Minus => UnaryOperator::Negate,
                Token::Bang => UnaryOperator::Not,
                _ => unreachable!(), // 因为上面已经匹配过了
            };

            let operand = self.parse_unary_expression()?;

            let location = self.previous().location;
            let location_line = location.line;
            let location_column = location.column;
            let location_file = location.file.to_string();

            Ok(Expr::Unary {
                location_line,
                location_column,
                location_file,
                operator,
                operand: Box::new(operand),
            })
        } else {
            self.parse_primary_expression()
        }
    }

    /// 解析主要表达式（字面量、标识符、括号表达式等）
    fn parse_primary_expression(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[Token::True, Token::False]) {
            let token = self.previous().clone();
            let value = match token.data {
                Token::True => Literal::Boolean(true),
                Token::False => Literal::Boolean(false),
                _ => unreachable!(),
            };

            Ok(Expr::Literal {
                location_line: token.location.line,
                location_column: token.location.column,
                location_file: token.location.file.to_string(),
                value,
            })
        } else if self.match_token(&[
            Token::IntegerLiteral(String::new()),
            Token::FloatLiteral(String::new()),
        ]) {
            // 注意：这里我们只是匹配类型，实际值在下面获取
            self.current -= 1; // 回退以便正确获取值

            let token = self.advance().clone();
            let value = match &token.data {
                Token::IntegerLiteral(value) => Literal::Integer(value.clone()),
                Token::FloatLiteral(value) => Literal::Float(value.clone()),
                _ => unreachable!(),
            };

            Ok(Expr::Literal {
                location_line: token.location.line,
                location_column: token.location.column,
                location_file: token.location.file.to_string(),
                value,
            })
        } else if let Token::StringLiteral(value) = &self.peek().data {
            let value = value.clone();
            let token = self.advance().clone();

            Ok(Expr::Literal {
                location_line: token.location.line,
                location_column: token.location.column,
                location_file: token.location.file.to_string(),
                value: Literal::String(value),
            })
        } else if let Token::CharLiteral(value) = self.peek().data {
            let value = value;
            let token = self.advance().clone();

            Ok(Expr::Literal {
                location_line: token.location.line,
                location_column: token.location.column,
                location_file: token.location.file.to_string(),
                value: Literal::Char(value),
            })
        } else if let Token::Identifier(name) = &self.peek().data {
            let name = name.clone();
            let token = self.advance().clone();

            // 检查是否是效果操作调用 (effect.operation)
            if self.match_token(&[Token::Dot]) {
                if let Token::Identifier(operation) = &self.peek().data {
                    let operation = operation.clone();
                    self.advance(); // 消费操作名

                    self.consume(&Token::LParen, "Expected '(' after operation name")?;

                    let mut arguments = Vec::new();
                    if !self.check(&Token::RParen) {
                        loop {
                            arguments.push(self.parse_expression()?);

                            if !self.match_token(&[Token::Comma]) {
                                break;
                            }
                        }
                    }

                    self.consume(&Token::RParen, "Expected ')' after arguments")?;

                    Ok(Expr::EffectOperation {
                        location_line: token.location.line,
                        location_column: token.location.column,
                        location_file: token.location.file.to_string(),
                        effect: name,
                        operation,
                        arguments,
                    })
                } else {
                    Err(ParseError::UnexpectedToken(
                        "Expected operation name".to_string(),
                    ))
                }
            } else {
                Ok(Expr::Identifier {
                    location_line: token.location.line,
                    location_column: token.location.column,
                    location_file: token.location.file.to_string(),
                    name,
                })
            }
        } else if self.match_token(&[Token::LParen]) {
            let location = self.previous().location;
            let location_line = location.line;
            let location_column = location.column;
            let location_file = location.file.to_string();

            let expression = self.parse_expression()?;
            self.consume(&Token::RParen, "Expected ')' after expression")?;

            Ok(Expr::Grouping {
                location_line,
                location_column,
                location_file,
                expression: Box::new(expression),
            })
        } else {
            Err(ParseError::UnexpectedToken(format!(
                "{:?}",
                self.peek().data
            )))
        }
    }

    /// 解析二元操作符
    fn parse_binary_operator(&mut self) -> Result<BinaryOperator, ParseError> {
        match &self.peek().data {
            Token::Plus => Ok(BinaryOperator::Add),
            Token::Minus => Ok(BinaryOperator::Subtract),
            Token::Star => Ok(BinaryOperator::Multiply),
            Token::Slash => Ok(BinaryOperator::Divide),
            Token::Percent => Ok(BinaryOperator::Modulo),
            Token::EqualEqual => Ok(BinaryOperator::Equal),
            Token::BangEqual => Ok(BinaryOperator::NotEqual),
            Token::Less => Ok(BinaryOperator::Less),
            Token::LessEqual => Ok(BinaryOperator::LessEqual),
            Token::Greater => Ok(BinaryOperator::Greater),
            Token::GreaterEqual => Ok(BinaryOperator::GreaterEqual),
            Token::And => Ok(BinaryOperator::And),
            Token::Or => Ok(BinaryOperator::Or),
            _ => Err(ParseError::UnexpectedToken(format!(
                "{:?}",
                self.peek().data
            ))),
        }
    }

    /// 获取当前token的优先级
    fn get_precedence(&self) -> Option<u8> {
        match &self.peek().data {
            Token::Or => Some(1),
            Token::And => Some(2),
            Token::EqualEqual | Token::BangEqual => Some(3),
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual => Some(4),
            Token::Plus | Token::Minus => Some(5),
            Token::Star | Token::Slash | Token::Percent => Some(6),
            _ => None,
        }
    }

    /// 检查当前token是否匹配给定的token之一
    fn match_token(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// 检查当前token是否匹配给定的token
    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }

        // 对于带有值的token类型，我们只比较类型而不比较值
        match (token, &self.peek().data) {
            (Token::IntegerLiteral(_), Token::IntegerLiteral(_)) => true,
            (Token::FloatLiteral(_), Token::FloatLiteral(_)) => true,
            (Token::StringLiteral(_), Token::StringLiteral(_)) => true,
            (Token::CharLiteral(_), Token::CharLiteral(_)) => true,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            _ => token == &self.peek().data,
        }
    }

    /// 消费一个预期的token，如果当前token不匹配则返回错误
    fn consume(
        &mut self,
        token: &Token,
        message: &str,
    ) -> Result<Locatable<'a, Token>, ParseError> {
        if self.check(token) {
            Ok(self.advance().clone())
        } else {
            Err(ParseError::MissingToken(message.to_string()))
        }
    }

    /// 获取前一个token
    fn previous(&self) -> &Locatable<'a, Token> {
        &self.tokens[self.current - 1]
    }

    /// 获取当前token
    fn peek(&self) -> &Locatable<'a, Token> {
        &self.tokens[self.current]
    }

    /// 消费当前token并前进到下一个token
    fn advance(&mut self) -> &Locatable<'a, Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// 检查是否已到达token流的末尾
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek().data == Token::Eof
    }
}
