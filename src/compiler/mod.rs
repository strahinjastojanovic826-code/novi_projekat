pub mod ast;
pub mod evaluator;
pub mod parser;
pub mod token;

use ast::ASTNode;
use evaluator::Evaluator;
use parser::Parser;
use token::{Lexer, Token};

pub struct QuantumScriptCompiler {
    pub source_code: String,
    pub last_tokens: Vec<Token>,
    pub last_ast: Option<ASTNode>,
    pub evaluator: Evaluator,
}

impl QuantumScriptCompiler {
    pub fn new() -> Self {
        let default_code = "let x = 10 + 5 * 2;\nlet y = (x - 4) / 2;\nprint x;\nprint y;".to_string();
        Self {
            source_code: default_code,
            last_tokens: Vec::new(),
            last_ast: None,
            evaluator: Evaluator::new(),
        }
    }

    pub fn compile_and_run(&mut self) -> Result<Vec<String>, String> {
        self.evaluator = Evaluator::new();

        // 1. Lexer
        let mut lexer = Lexer::new(&self.source_code);
        self.last_tokens = lexer.tokenize();

        // 2. Parser -> AST
        let mut parser = Parser::new(self.last_tokens.clone());
        let ast = parser.parse()?;
        self.last_ast = Some(ast.clone());

        // 3. Evaluator
        self.evaluator.eval(&ast)?;

        Ok(self.evaluator.output.clone())
    }
}