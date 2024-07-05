use super::models::Sequence;
use crate::Range;





pub struct LinearCombination<'a, S1, S2> {
    seq1: &'a S1,
    seq2: &'a S2,
    a: f64,
    b: f64,
}

impl<'a, S1: Sequence<f64>, S2: Sequence<f64>> LinearCombination<'a, S1, S2> {
    fn new(seq1: &'a S1, seq2: &'a S2, a: f64, b: f64) -> LinearCombination<'a, S1, S2> {
        LinearCombination { seq1, seq2, a, b }
    }

    fn k_th(&self, k: usize) -> f64 {
        self.a * self.seq1.k_th(k) + self.b * self.seq2.k_th(k)
    }

    fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k <= range.to {
            result.push(self.k_th(k.try_into().unwrap()));
            k += range.step;
        }
        result
    }
}