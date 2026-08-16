use std::collections::HashMap;
use crate::compiler::ast::ASTNode;

pub struct Evaluator {
    pub env: HashMap<String, f64>,
    pub output: Vec<String>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: HashMap::new(),
            output: Vec::new(),
        }
    }

    pub fn eval(&mut self, node: &ASTNode) -> Result<f64, String> {
        match node {
            ASTNode::Number(val) => Ok(*val),
            ASTNode::Variable(name) => {
                if let Some(val) = self.env.get(name) {
                    Ok(*val)
                } else {
                    Err(format!("Nedefinisana promenljiva: '{}'", name))
                }
            }
            ASTNode::BinaryOp { op, left, right } => {
                let l_val = self.eval(left)?;
                let r_val = self.eval(right)?;
                match op {
                    '+' => Ok(l_val + r_val),
                    '-' => Ok(l_val - r_val),
                    '*' => Ok(l_val * r_val),
                    '/' => {
                        if r_val == 0.0 {
                            Err("Deljenje sa nulom nije dozvoljeno!".into())
                        } else {
                            Ok(l_val / r_val)
                        }
                    }
                    _ => Err(format!("Nepoznat operator: {}", op)),
                }
            }
            ASTNode::VarDecl { name, value } => {
                let val = self.eval(value)?;
                self.env.insert(name.clone(), val);
                Ok(val)
            }
            ASTNode::Print(expr) => {
                let val = self.eval(expr)?;
                self.output.push(format!("> {}", val));
                Ok(val)
            }
            ASTNode::Program(statements) => {
                let mut last = 0.0;
                for stmt in statements {
                    last = self.eval(stmt)?;
                }
                Ok(last)
            }
        }
    }
}