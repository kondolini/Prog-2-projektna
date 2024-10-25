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

}


impl Sequence<f64> for Geometric {
    fn k_th(&self, k: usize) -> f64 {
        self.zac_clen * self.kvocient.powi(k as i32)
    }

}

