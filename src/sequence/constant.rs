use super::models::Sequence;
use crate::Range;


pub struct Constant {
    name:String,
    vrednost : f64,

}

impl Constant {
   pub fn new(name:String, vrednost:f64) -> Constant {
    Constant{name, vrednost}
   }
    
    pub fn k_th(&self, k: u64) -> f64 {
        self.vrednost
    }
    pub fn start(&self) -> f64 {
        self.vrednost
    }
    pub fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k <= range.to {
            result.push(self.k_th(k));
            k += range.step;
        }
        result
    }
}
impl Sequence<f64> for Constant {
    fn k_th(&self, _k:usize) -> f64 {
        self.vrednost
    }
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn start(&self) -> f64 {
        self.vrednost
    }
}



