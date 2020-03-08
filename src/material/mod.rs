pub mod diffuse;
pub mod specular;

use super::color::Color;
use super::hit::Hit;
use super::random::Rng;
use super::ray::Ray;

pub trait Material: Sync {
    fn sample<'a>(
        &self,
        random: &'a mut Box<dyn Rng>,
        hit: Hit,
        sampler: Box<dyn MaterialSampler + 'a>,
    ) -> Color;
}

pub trait MaterialSampler: Sync {
    fn sample<'a>(&self, random: &'a mut Box<dyn Rng>, ray: Ray) -> Color;
}
