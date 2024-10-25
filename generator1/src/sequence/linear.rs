use std::string;

use super::models::Sequence;
use crate::Range;





pub struct LinearCombination {
    name: String,                 
    seq1: Box<dyn Sequence<f64>>, 
    seq2: Box<dyn Sequence<f64>>, 
    a: f64,                       
    b: f64,                       
}

impl LinearCombination {
    pub fn new(name:String, seq1: Box<dyn Sequence<f64>>, seq2: Box<dyn Sequence<f64>>, a: f64, b: f64) -> LinearCombination {
        LinearCombination {name, seq1, seq2, a, b }
    }

}


impl Sequence<f64> for LinearCombination {
    fn k_th(&self, k: usize) -> f64 {
        self.a * self.seq1.k_th(k) + self.b * self.seq2.k_th(k)
    }

}
