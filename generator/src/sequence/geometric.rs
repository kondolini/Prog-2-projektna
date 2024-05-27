use super::models::Sequence;
// Implementirajte geometrijsko zaporedje
pub struct Geometric <f64> {
    zacetni : f64,
    kolicnik: f64,

}

impl Sequence<f64> for Geometric<f64> {
    fn name(&self) -> String {
        format!("geometrijsko, z začetnim členom {} in količnikom {}",self.zacetni,self.kolicnik)
    }
    fn contains(&self, item: f64) -> bool {
        let mut count = self.zacetni;
        loop {
            if count > item {
                return false
            }
            if count == item {
                return true
            }
            count += self.diferenca;
        }
    }
    fn k_th(&self, k: usize) -> Option<f64> {
        Some(self.zacetni * (self.kolicnik).pow(k))
    }
    fn start(&self) -> f64 {
        self.zacetni
    }
}
impl Geometric<f64> {
    pub fn new(x: f64, y: f64) -> Geometric<f64> {
        Geometric {zacetni: x, kolicnik: y}
    }
}
