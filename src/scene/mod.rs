mod aabb;
mod intersection;
mod obj;
mod tree;
mod triangle;

use super::color::Color;
use super::random;
use super::ray::Ray;

// Scene is a collection of geometry.
pub trait Scene {
    fn sample(&self, random: &mut Box<dyn random::Rng>, ray: Ray, bounce_depth: usize) -> Color;
}
