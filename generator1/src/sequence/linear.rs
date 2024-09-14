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

    pub fn k_th(&self, k: usize) -> f64 {
        self.a * self.seq1.k_th(k) + self.b * self.seq2.k_th(k)
    }

    pub fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k <= range.to {
            result.push(self.k_th(k.try_into().unwrap()));
            k += range.step;
        }
        result
    }
}


impl Sequence<f64> for LinearCombination {
    fn k_th(&self, k: usize) -> f64 {
        self.a * self.seq1.k_th(k) + self.b * self.seq2.k_th(k)
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn start(&self) -> f64 {
        self.k_th(0)
    }
}
