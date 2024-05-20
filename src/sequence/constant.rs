use crate::sequence::models::Sequence;
use std::any::type_name;

// Implementirajte konstantno zaporedje
pub struct Constant <f64> {
    vrednost : f64,

}

impl Sequence<f64> for Constant<f64> {
    fn name(&self) -> String {
        format!("Const: {}",self.vrednost)
    }
    fn contains(&self, item: f64) -> bool {
        if item == self.vrednost {
            true
        }
        else {false}
    }
    fn k_th(&self, k: usize) -> Option<f64> {
        Some(self.vrednost)
    }
    fn start(&self) -> f64 {
        self.vrednost
    }
}
impl Constant<f64> {
    pub fn new(x: f64) -> Constant<f64> {
        Constant {vrednost: x}
    }
}


