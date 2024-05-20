use super::models::Sequence;
// Implementirajte artimetično zaporedje
pub struct Arithmetic <f64> {
    zacetni : f64,
    diferenca: f64,

}

impl Sequence<f64> for Arithmetic<f64> {
    fn name(&self) -> String {
        format!("arimetično, z začetnim členom {} in diferenco {}",self.zacetni,self.diferenca)
    }
    fn contains(&self, item: f64) -> bool {
        let mut count = self.zacetni;
        loop {
            if count > item {
                return false
            }
            if count == item {
                return false
            }
            count += self.diferenca;
        }
    }
    fn k_th(&self, k: usize) -> Option<f64> {
        Some(self.zacetni + (self.diferenca)*(k as f64))
    }
    fn start(&self) -> f64 {
        self.zacetni
    }
}
impl Arithmetic<f64> {
    pub fn new(x: f64, y: f64) -> Arithmetic<f64> {
        Arithmetic {zacetni: x, diferenca: y}
    }
}
