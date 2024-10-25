use crate::Range;
use super::models::Sequence;

pub struct Drop {
    name: String,
    sequence: Box<dyn Sequence<f64>>,     
    count: usize,
}


impl Drop

{
    
    pub fn new(name:String,sequence: Box<dyn Sequence<f64>>, count: usize) -> Drop {
        Drop { name, sequence, count }
    }
   

}

impl Sequence<f64> for Drop {
    fn k_th(&self, k: usize) -> f64 {
        self.sequence.k_th(k + self.count)
    }
}

