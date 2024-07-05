use super::models::Sequence;
use crate::Range;


pub struct Constant {
    vrednost : f64,

}

impl Constant {
   
    fn contains(&self, item: f64) -> bool {
        if item == self.vrednost {
            true
        }
        else {false}
    }
    fn k_th(&self, k: u64) -> f64 {
        self.vrednost
    }
    fn start(&self) -> f64 {
        self.vrednost
    }
    fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k <= range.to {
            result.push(self.k_th(k));
            k += range.step;
        }
        result
    }
}



