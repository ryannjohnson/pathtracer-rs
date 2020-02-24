use super::color::Color;
use super::hit::Hit;
use super::ray::Ray;
use super::random as local_random;

pub trait Material {
    fn sample<'a>(&self, random: &'a Box<dyn local_random::Rng + 'a>, hit: Hit, sampler: Box<dyn Sampler + 'a>) -> Color;
}

pub trait Sampler {
    fn sample(&self, ray: Ray) -> Color;
}
