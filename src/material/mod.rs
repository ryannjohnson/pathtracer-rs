pub mod diffuse;
pub mod specular;

use super::color::Color;
use super::hit::Hit;
use super::ray::Ray;
use super::random::Rng;

pub trait Material {
    fn sample<'a>(&self, random: &'a mut Box<dyn Rng>, hit: Hit, sampler: Box<dyn FnMut(Ray) -> Color + 'a>) -> Color;
}
