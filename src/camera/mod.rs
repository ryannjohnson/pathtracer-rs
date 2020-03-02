pub mod perspective;

use super::ray::Ray;
use super::random::Rng;

pub trait Camera {
    fn cast(&self, random: &mut impl Rng, x: f64, y: f64) -> Ray;
}
