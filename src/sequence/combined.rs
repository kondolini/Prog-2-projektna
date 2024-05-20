use crate::expression::models::AExpr;
use crate::sequence::models::Sequence;
use std::collections::HashMap;

pub struct Combined<'a, T> {
    samo_da_ni_unused_variable: Box<&'a T>,
}

impl Sequence<i64> for Combined<'_, i64> {
    fn name(&self) -> String {
        panic!("Shifted")
    }

    fn start(&self) -> i64 {
        panic!("Shifted")
    }

    fn k_th(&self, k: usize) -> Option<i64> {
        panic!("Shifted")
    }

    fn contains(&self, item: i64) -> bool {
        panic!("Shifted")
    }
}

impl<T> Combined<'_, T> {
    fn new(sequences: Vec<Box<&dyn Sequence<T>>>, expression: AExpr) -> Box<Combined<T>> {
        Box::new(panic!("Shifted"))
    }
}

pub fn combined_sequence(
    sequences: Vec<Box<&dyn Sequence<i64>>>,
    expression: AExpr,
) -> Box<dyn Sequence<i64> + '_> {
    Combined::new(sequences, expression)
}

fn main() {
    yo();
}

fn yo (){
    let mut list = vec![1,2,3];
    let f = ||println!("tukaj je list {:?}", list);
    f();
    list.push(4);
    f()
}