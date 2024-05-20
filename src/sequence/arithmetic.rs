use super::models::Sequence;
// Implementirajte artimetično zaporedje
pub struct Arithmetic <i64> {
    zacetni : i64,
    diferenca: i64,

}

impl Sequence<i64> for Arithmetic<i64> {
    fn name(&self) -> String {
        format!("arimetično, z začetnim členom {} in diferenco {}",self.zacetni,self.diferenca)
    }
    fn contains(&self, item: i64) -> bool {
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
    fn k_th(&self, k: usize) -> Option<i64> {
        Some(self.zacetni + (self.diferenca)*(k as i64))
    }
    fn start(&self) -> i64 {
        self.zacetni
    }
}
impl Arithmetic<i64> {
    pub fn new(x: i64, y: i64) -> Arithmetic<i64> {
        Arithmetic {zacetni: x, diferenca: y}
    }
}
