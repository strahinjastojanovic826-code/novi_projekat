use crate::compiler::ast::ASTNode;
use crate::compiler::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let mut stmts = Vec::new();

        while self.current() != &Token::Eof {
            stmts.push(self.parse_statement()?);
        }

        Ok(ASTNode::Program(stmts))
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        match self.current() {
            Token::Let => {
                self.advance();
                let name = match self.current() {
                    Token::Identifier(id) => id.clone(),
                    _ => return Err("Očekivano ime promenljive nakon 'let'".into()),
                };
                self.advance();

                if self.current() != &Token::Assign {
                    return Err("Očekivan '=' znak za dodelu".into());
                }
                self.advance();

                let val = self.parse_expression()?;
                if self.current() == &Token::Semicolon {
                    self.advance();
                }

                Ok(ASTNode::VarDecl {
                    name,
                    value: Box::new(val),
                })
            }
            Token::Print => {
                self.advance();
                let expr = self.parse_expression()?;
                if self.current() == &Token::Semicolon {
                    self.advance();
                }
                Ok(ASTNode::Print(Box::new(expr)))
            }
            _ => {
                let expr = self.parse_expression()?;
                if self.current() == &Token::Semicolon {
                    self.advance();
                }
                Ok(expr)
            }
        }
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_term()?;

        while matches!(self.current(), Token::Plus | Token::Minus) {
            let op = if self.current() == &Token::Plus { '+' } else { '-' };
            self.advance();
            let right = self.parse_term()?;
            left = ASTNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_factor()?;

        while matches!(self.current(), Token::Star | Token::Slash) {
            let op = if self.current() == &Token::Star { '*' } else { '/' };
            self.advance();
            let right = self.parse_factor()?;
            left = ASTNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, String> {
        match self.current().clone() {
            Token::Number(val) => {
                self.advance();
                Ok(ASTNode::Number(val))
            }
            Token::Identifier(name) => {
                self.advance();
                Ok(ASTNode::Variable(name))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                if self.current() != &Token::RParen {
                    return Err("Očekivana zatvorena zagrada ')'".into());
                }
                self.advance();
                Ok(expr)
            }
            _ => Err(format!("Sintaksna greška u blizini tokena {:?}", self.current())),
        }
    }
}