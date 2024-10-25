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

    
    }


impl Sequence<f64> for Produkt
{
    fn k_th(&self, k: usize) -> f64 {
        self.seq1.k_th(k) * self.seq2.k_th(k)
    }
}