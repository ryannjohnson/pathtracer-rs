use super::Rng;
use rand;
use std;

#[derive(Clone, Copy)]
pub struct ThreadRng {
    rng: rand::rngs::ThreadRng,
}

impl ThreadRng {
    pub fn new() -> ThreadRng {
        ThreadRng {
            rng: rand::thread_rng(),
        }
    }
}

impl Rng for ThreadRng {
    fn next_f64(&mut self) -> f64 {
        let a = rand::RngCore::next_u64(&mut self.rng);
        let mut b = a as f64 / std::f64::MAX;

        if b == std::f64::MAX {
            // TODO: Avoid this edge case somehow.
            b = b - std::f64::EPSILON;
        }

        b
    }
}
