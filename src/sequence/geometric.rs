use super::models::Sequence;
// Implementirajte geometrijsko zaporedje
use crate::Range;

// Definirajte strukturo Geometric brez Box
pub struct Geometric {
    name: String,
    zac_clen: f64,
    kvocient: f64,
}

impl Geometric {
    // Konstruktor za ustvarjanje novega geometrijskega zaporedja
    pub fn new(name: String,zac_clen: f64, kvocient: f64) -> Geometric {
        Geometric {name, zac_clen, kvocient }
    }

    // Metoda za pridobitev k-tega člena zaporedja
    pub fn k_th(&self, k: usize) -> f64 {
        self.zac_clen * self.kvocient.powi(k as i32)
    }

    // Metoda za pridobitev zaporedja v določenem razponu
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


impl Sequence<f64> for Geometric {
    fn k_th(&self, k: usize) -> f64 {
        self.zac_clen * self.kvocient.powi(k as i32)
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn start(&self) -> f64 {
        self.k_th(0)
    }

}

