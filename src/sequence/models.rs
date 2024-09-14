
pub trait Sequence<T> {
    fn name(&self) -> String;
    fn start(&self) -> T;

    // To pustimo do naslednjič, ko se bom natančneje poučili o Rustovih traitih in izposojanju
    // fn current_index(&self) -> usize;
    // fn current(&self) -> Option<T>;

    // fn next(&mut self) -> Option<T>;
    // fn k_next(&mut self, k: usize) -> Option<T>;

    fn k_th(&self, k: usize) -> f64;

}
