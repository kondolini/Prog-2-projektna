use crate::Range;

pub trait Sequence<T> {
    fn k_th(&self, k: usize) -> f64;

    fn name(&self) -> String {
        self.name().to_string()
}
    fn start(&self) -> f64 {
        self.k_th(0)
    }
    fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut k = range.from;
        while k <= range.to {
            result.push(self.k_th(k.try_into().unwrap()));
            k += range.step;
        }
        result
    }
}
