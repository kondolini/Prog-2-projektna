use std::f64;
use std::collections::HashMap;
use super::models::Sequence;
use crate::Range;


pub struct PowerSequence<'a, S1, S2> {
    s1: &'a S1,
    s2: &'a S2,
}

impl<'a, S1: Sequence<f64>, S2: Sequence<f64>> PowerSequence<'a, S1, S2>

{
    pub fn new(s1: &'a S1, s2: &'a S2) -> Self {
        PowerSequence { s1, s2 }   }





    pub fn k_th(&self, k: usize) -> f64 {
        let value_s1 = self.s1.k_th(k);
        let value_s2 = self.s2.k_th(k);
        (value_s1.abs().powf(value_s2)).clamp(-7.0, f64::INFINITY)
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
