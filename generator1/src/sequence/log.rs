use std::f64;
use std::collections::HashMap;
use super::models::Sequence;
use crate::Range;

pub struct LogSequence {
    name: String,                 
    seq1: Box<dyn Sequence<f64>>, 
    seq2: Box<dyn Sequence<f64>>, 
}

impl LogSequence {
    pub fn new(name: String,seq1: Box<dyn Sequence<f64>>, 
        seq2: Box<dyn Sequence<f64>>) -> Self {
        LogSequence { name,seq1, seq2 }
    }

    pub fn k_th(&self, k: usize) -> f64 {
        let value_s1 = self.seq1.k_th(k);
        let value_s2 = self.seq2.k_th(k);
        if value_s1 > 0.0 && value_s2 > 1.0 && value_s2 != 1.0{
            value_s1.log(value_s2)
        } else {
            f64::NAN 
        }
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

impl Sequence<f64> for LogSequence {
    fn k_th(&self, k: usize) -> f64 {
        let value_s1 = self.seq1.k_th(k);
        let value_s2 = self.seq2.k_th(k);
        if value_s1 > 0.0 && value_s2 > 1.0 {
            value_s1.log(value_s2)
        } else {
            f64::NAN 
        }
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn start(&self) -> f64 {
        self.k_th(0)
    }
}
