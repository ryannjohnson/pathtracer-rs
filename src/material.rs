use super::color::Color;
use super::hit::Hit;
use super::ray::Ray;
use rand::RngCore;

pub trait Material {
    fn sample(&self, random: impl RngCore, hit: Hit, sampler: Sampler) -> Color;
}

type Sampler = fn(ray: Ray) -> Color;
