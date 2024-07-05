use crate::Range;
use super::models::Sequence;

pub struct Drop<S> {
    sequence: S,     
    count: usize,    
}


impl<Sequence<f64> Drop<Sequence<f64> 

{
    
    pub fn new(sequence: S, count: usize) -> Drop<S> {
        Drop { sequence, count }
    }

    
    pub fn k_th(&self, k: usize) -> f64 {         
        self.sequence.k_th(k + self.count)
    }
    

    
    pub fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from + self.count;
        while k <= range.to + self.count {
            result.push(self.sequence.k_th(k as usize));
            k += range.step;
        }
        result
    }
}


