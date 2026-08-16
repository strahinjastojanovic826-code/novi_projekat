#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f64),
    Variable(String),
    BinaryOp {
        op: char,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    VarDecl {
        name: String,
        value: Box<ASTNode>,
    },
    Print(Box<ASTNode>),
    Program(Vec<ASTNode>),
}