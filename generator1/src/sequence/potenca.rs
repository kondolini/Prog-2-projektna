use std::f64;
use super::models::Sequence;
use crate::Range;


pub struct PowerSequence{
    name: String,
    seq1: Box<dyn Sequence<f64>>,
    seq2: Box<dyn Sequence<f64>>,
}

impl PowerSequence
{
    pub fn new(name: String, seq1: Box<dyn Sequence<f64>>, seq2:Box<dyn Sequence<f64>>) -> Self {
        PowerSequence { name, seq1, seq2 }   }


    
    }

impl Sequence<f64> for PowerSequence
{
    fn k_th(&self, k: usize) -> f64 {
        let value_s1 = self.seq1.k_th(k);
        let value_s2 = self.seq2.k_th(k);
        (value_s1.abs().powf(value_s2)).clamp(-7.0, f64::INFINITY)
    }
}