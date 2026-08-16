#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Let,
    Print,
    Plus,
    Minus,
    Star,
    Slash,
    Assign,
    LParen,
    RParen,
    Semicolon,
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pos < self.input.len() {
            let ch = self.input[self.pos];

            if ch.is_whitespace() {
                self.pos += 1;
                continue;
            }

            if ch.is_ascii_digit() {
                tokens.push(self.read_number());
                continue;
            }

            if ch.is_alphabetic() || ch == '_' {
                tokens.push(self.read_identifier());
                continue;
            }

            let tok = match ch {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,
                '=' => Token::Assign,
                '(' => Token::LParen,
                ')' => Token::RParen,
                ';' => Token::Semicolon,
                _ => {
                    self.pos += 1;
                    continue;
                }
            };

            tokens.push(tok);
            self.pos += 1;
        }

        tokens.push(Token::Eof);
        tokens
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos].is_ascii_digit() || self.input[self.pos] == '.') {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        Token::Number(s.parse::<f64>().unwrap_or(0.0))
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_') {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        match s.as_str() {
            "let" => Token::Let,
            "print" => Token::Print,
            _ => Token::Identifier(s),
        }
    }
}