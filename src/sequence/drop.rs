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
   
    
    pub fn k_th(&self, k: usize) -> f64 {         
        self.sequence.k_th(k + self.count)
    }
    

    
    pub fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from + self.count as u64;
        while k <= range.to + self.count as u64{
            result.push(self.sequence.k_th(k as usize));
            k += range.step;
        }
        result
    }
}

impl Sequence<f64> for Drop {
    fn k_th(&self, k: usize) -> f64 {
        self.sequence.k_th(k + self.count)
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn start(&self) -> f64 {
        self.k_th(0)
    }

}

