use super::models::Sequence;
use crate::Range;





pub struct LinearCombination<S1, S2> {
    seq1: S1,
    seq2: S2,
    a: f64,
    b: f64,
}


impl<S1: Sequence<f64>, S2: Sequence<f64>> LinearCombination<S1, S2>

{
    
    pub fn new(seq1: S1, seq2: S2, a: f64, b: f64) -> LinearCombination<S1, S2> {
        LinearCombination { seq1, seq2, a, b }
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