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
    pub fn start(&self) -> f64 {
        self.vrednost
    }
  
}
impl Sequence<f64> for Constant {
    fn k_th(&self, _k:usize) -> f64 {
        self.vrednost
    }

}



