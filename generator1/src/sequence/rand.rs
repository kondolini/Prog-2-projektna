use rand::{thread_rng, Rng};
use super::models::Sequence;
use crate::Range;

pub struct ProbabilisticSequence<'a, S1, S2> {
    s1: &'a S1,
    s2: &'a S2,
    probability: f64,
}

impl<'a, S1: Sequence<f64>, S2: Sequence<f64>> ProbabilisticSequence<'a, S1, S2> {
    pub fn new(s1: &'a S1, s2: &'a S2, probability: f64) -> Self {
        if probability < 0.0 || probability > 1.0 {
            panic!("Probability must be between 0 and 1.");
        }
        ProbabilisticSequence { s1, s2, probability }
    }

    pub fn k_th(&self, k: usize) -> f64 {
        let mut rng = thread_rng();
        let random: f64 = rng.gen_range(0.0..1.0);
        if random < self.probability {
            self.s1.k_th(k)
        } else {
            self.s2.k_th(k)
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

