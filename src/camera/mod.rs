pub mod perspective;

use super::random::Rng;
use super::ray::Ray;

pub trait Camera: Sync {
    fn cast(&self, random: &mut impl Rng, x: f64, y: f64) -> Ray;
}
