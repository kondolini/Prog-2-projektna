use super::models::{AExpr, BinaryOperation};

impl AExpr {
    pub fn evaluate(&self) -> i64 {
        match self {
            AExpr::Num(x) => *x,
            AExpr::BinOp(prva, operacija, druga) => match operacija {
                BinaryOperation::Add => prva.evaluate() + druga.evaluate(),
                BinaryOperation::Sub => prva.evaluate() - druga.evaluate(),
                BinaryOperation::Mul => prva.evaluate() * prva.evaluate(),
                BinaryOperation::Pow => i64::pow(prva.evaluate(), druga.evaluate().try_into().unwrap())
            }
            AExpr::Variable(x) => 1
    

        }
    }

    pub fn evaluate_map(&self, vars: &std::collections::HashMap<String, Option<i64>>) -> Option<i64> {
        match self {
            AExpr::Num(x) => Some(*x),
            AExpr::BinOp(prva, operacija, druga) => match operacija {
                BinaryOperation::Add => Some(prva.evaluate() + druga.evaluate()),
                BinaryOperation::Sub => Some(prva.evaluate() - druga.evaluate()),
                BinaryOperation::Mul => Some(prva.evaluate() * prva.evaluate()),
                BinaryOperation::Pow => Some(i64::pow(prva.evaluate(), druga.evaluate().try_into().unwrap()))
            }
            AExpr::Variable(x) => *vars.get(x).unwrap()
        }
    }
}
