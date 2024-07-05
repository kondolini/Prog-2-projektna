use super::models::Sequence;
use crate::Range;

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct OperationSequence<'a, S1, S2, S3> {
    s1: &'a S1,
    s2: &'a S2,
    s3: &'a S3,
    operation: Operation,
    c: f64,
}

impl<'a, S1: Sequence<f64>, S2: Sequence<f64>, S3: Sequence<f64>> OperationSequence<'a, S1, S2, S3> {
    pub fn new(s1: &'a S1, s2: &'a S2, s3: &'a S3, operation: Operation, c: f64) -> Self {
        OperationSequence { s1, s2, s3, operation, c }
    }

    fn apply_operation(&self, value1: f64, value2: f64) -> f64 {
        match self.operation {
            Operation::Add => value1 + value2,
            Operation::Subtract => value1 - value2,
            Operation::Multiply => value1 * value2,
            Operation::Divide => {
                if value2 != 0.0 {
                    value1 / value2
                } else {
                    f64::NAN
                }
            }
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