use crate::expression::models::{AExpr, BinaryOperation};
use crate::expression::{evaluation, models};
use crate::sequence::arithmetic::Arithmetic;
use crate::sequence::combined::Combined;
use crate::sequence::models::Sequence;
use crate::sequence::constant::Constant;

pub mod expression;
pub mod sequence;

use std::collections::HashMap;

fn main() {
    // Naredite nekaj zaporedij
    let s1 = sequence::constant::Constant::new(1);
    let s2 = sequence::constant::Constant::new(2);
    let s3 = sequence::arithmetic::Arithmetic::new(0, 10);
    // let s4 = sequence::shifted::shifted_sequence(&*s3, 5);

    //println!("{}", s3.name());

    //println!("{:?}", s1.k_th(10));
    //println!("{:?}", s3.name());

    // Kombinirano zaporedje

    let neki = AExpr::BinOp(
         Box::new(AExpr::Variable(s3.name())),
         BinaryOperation::Add,
         Box::new(AExpr::BinOp(
             Box::new(AExpr::Num(2)),
             BinaryOperation::Mul,
             Box::new(AExpr::Num(3)),
         )),
     );
     println!("{:?}",neki)

    // Najlažji način, da pravilno zamenjamo tip in ga damo v vector
    // let s3t: &dyn Sequence<i64> = &*s3_;
    // let zap = sequence::combined::combined_sequence(vec![Box::new(s3t)], neki);

    // println!("{:?}", zap.k_th(0));
    // println!("{:?}", zap.k_th(1));
    // println!("{:?}", zap.k_th(2));
    // println!("{:?}", zap.name());
}
