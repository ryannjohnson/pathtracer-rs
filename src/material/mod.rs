pub mod diffuse;
pub mod specular;

use super::color::Color;
use super::hit::Hit;
use super::ray::Ray;
use super::random as local_random;

pub trait Material {
    fn sample<'a>(&self, random: &'a mut Box<dyn local_random::Rng>, hit: Hit, sampler: Box<dyn Fn(Ray) -> Color + 'a>) -> Color;
}
