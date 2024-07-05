use super::models::Sequence;
// Implementirajte geometrijsko zaporedje
use crate::Range;

// Definirajte strukturo Geometric brez Box
pub struct Geometric {
    zac_clen: f64,
    kvocient: f64,
}

impl Geometric {
    // Konstruktor za ustvarjanje novega geometrijskega zaporedja
    pub fn new(zac_clen: f64, kvocient: f64) -> Geometric {
        Geometric { zac_clen, kvocient }
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

// Primer uporabe
fn main() {
    let geometric_sequence = Geometric::new(1.0, 2.0); // Začnite z 1 in kvocientom 2
    let range = Range { from: 0, to: 10, step: 1 };

    let sequence = geometric_sequence.range(range);
    for value in sequence {
        println!("{}", value);
    }
}
