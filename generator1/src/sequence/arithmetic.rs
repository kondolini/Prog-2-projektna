use super::models::Sequence;
use crate::Range;
// Implementirajte artimetično zaporedje
pub struct Arithmetic {
    name: String,
    start: f64,
    step: f64,
}

impl Arithmetic {
    pub fn new(name:String,start: f64, step: f64) -> Arithmetic {
       Arithmetic { name,start, step }
    }

}

impl Sequence<f64> for Arithmetic {
    fn k_th(&self, k: usize) -> f64 {
        self.start + (k as f64) * self.step
    }
}