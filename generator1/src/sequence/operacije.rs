use super::models::Sequence;
use crate::Range;

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct OperationSequence {
    name: String,
    s1: Box<dyn Sequence<f64>>,
    s2: Box<dyn Sequence<f64>>,
    s3: Box<dyn Sequence<f64>>,
    operation: f64,
    c: f64,
}

impl OperationSequence {
    pub fn new(name: String,s1:  Box<dyn Sequence<f64>>, s2:Box<dyn Sequence<f64>>, s3:Box<dyn Sequence<f64>>, operation: f64, c: f64) -> Self {
        OperationSequence { name,s1, s2, s3, operation, c }
    }

    fn apply_operation(&self, value1: f64, value2: f64) -> f64 {
        match self.operation {
            1. => value1 + value2,
            2. => value1 - value2,
            3. => value1 * value2,
            4. => {
                if value2 != 0.0 {
                    value1 / value2
                } else {
                    f64::NAN
                }
            }
            _ => value1 + value2
        }
    }

    pub fn k_th(&self, k: usize) -> f64 {
        let value_s1 = self.s1.k_th(k);
        let value_s2 = self.s2.k_th(k);
        let value_s3 = self.s3.k_th(k);

        let a = self.apply_operation(value_s1, value_s2);
        let b = self.apply_operation(value_s1, value_s3);

        let diff_a = (a - self.c).abs();
        let diff_b = (b - self.c).abs();

        if diff_a < diff_b {
            a
        } else {
            b
        }
    }

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

impl Sequence<f64> for OperationSequence {
    fn k_th(&self, k: usize) -> f64 {
        let value_s1 = self.s1.k_th(k);
        let value_s2 = self.s2.k_th(k);
        let value_s3 = self.s3.k_th(k);

        let a = self.apply_operation(value_s1, value_s2);
        let b = self.apply_operation(value_s1, value_s3);

        let diff_a = (a - self.c).abs();
        let diff_b = (b - self.c).abs();

        if diff_a < diff_b {
            a
        } else {
            b
        }
    }
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn start(&self) -> f64 {
        self.k_th(0)
    }
    }

