use crate::sequence::models::Sequence;
use std::any::type_name;

// Implementirajte konstantno zaporedje
pub struct Constant <i64> {
    vrednost : i64,

}

impl Sequence<i64> for Constant<i64> {
    fn name(&self) -> String {
        format!("Const: {}",self.vrednost)
    }
    fn contains(&self, item: i64) -> bool {
        if item == self.vrednost {
            true
        }
        else {false}
    }
    fn k_th(&self, k: usize) -> Option<i64> {
        Some(self.vrednost)
    }
    fn start(&self) -> i64 {
        self.vrednost
    }
}
impl Constant<i64> {
    pub fn new(x: i64) -> Constant<i64> {
        Constant {vrednost: x}
    }
}


