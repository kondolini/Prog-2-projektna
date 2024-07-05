use super::models::Sequence;
use crate::Range;
// Implementirajte artimetično zaporedje
pub struct Arithmetic {
    start: f64,
    step: f64,
}

impl Arithmetic {
    pub fn new(start: f64, step: f64) -> Arithmetic {
       Arithmetic { start, step }
    }

    pub fn k_th(&self, k: usize) -> f64 {
        self.start + (k as f64) * self.step
    }

    pub fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k <= range.to {
            result.push(self.k_th(k as usize));
            k += range.step;
        }
        result
    }
}