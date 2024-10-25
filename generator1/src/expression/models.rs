#[derive(Debug)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Pow,
}

#[derive(Debug)]
pub enum AExpr {
    Num(i64),
    Variable(String),
    BinOp(Box<AExpr>, BinaryOperation, Box<AExpr>),
}
