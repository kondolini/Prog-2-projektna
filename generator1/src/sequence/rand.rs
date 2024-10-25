use rand::{thread_rng, Rng};
use super::models::Sequence;
use crate::Range;

pub struct ProbabilisticSequence{
    name: String,
    seq1: Box<dyn Sequence<f64>>,
    seq2: Box<dyn Sequence<f64>>,
    probability: f64
}

impl ProbabilisticSequence {
    pub fn new(name: String ,seq1:Box<dyn Sequence<f64>>, seq2: Box<dyn Sequence<f64>>, probability: f64) -> Self {
        if probability < 0.0 || probability > 1.0 {
            panic!("Probability must be between 0 and 1.");
        }
        ProbabilisticSequence { name, seq1, seq2, probability}
    }

   }

impl Sequence<f64> for ProbabilisticSequence
{
    fn k_th(&self, k: usize) -> f64 {
        let mut rng = thread_rng();
        let random: f64 = rng.gen_range(0.0..1.0);
        if random < self.probability {
            self.seq1.k_th(k)
        } else {
            self.seq2.k_th(k)
        }
    }
}