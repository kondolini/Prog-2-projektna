use crate::Range;
use super::models::Sequence;


pub struct Produkt{
    name: String,
    seq1: Box<dyn Sequence<f64>>,
    seq2: Box<dyn Sequence<f64>>,
}
impl Produkt {
   
    pub fn new(name: String, seq1: Box<dyn Sequence<f64>>, seq2:Box<dyn Sequence<f64>>) -> Self {
        Produkt { name, seq1, seq2 }   }

    
    pub fn k_th(&self, k: usize) -> f64 {
        self.seq1.k_th(k) * self.seq2.k_th(k)
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


impl Sequence<f64> for Produkt
{
    fn k_th(&self, k: usize) -> f64 {
        self.seq1.k_th(k) * self.seq2.k_th(k)
    }

    fn name(&self) -> String {
        self.name.to_string()
}
    fn start(&self) -> f64 {
        self.k_th(0)
    }
}