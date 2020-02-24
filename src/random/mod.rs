pub mod thread;

pub trait Rng {
    /// Within a range of [0, 1.0).
    fn next_f64(&mut self) -> f64;
}
