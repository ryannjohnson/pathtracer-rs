mod aabb;
mod intersection;
mod obj;
mod tree;
mod triangle;

use super::color::Color;
use super::hit::Hit;
use super::material::Material;
use super::ray::Ray;

// Scene is a collection of geometry.
pub trait Scene {
    fn intersect<'a>(&self, ray: Ray, callback: Box<dyn Fn(SceneHit) -> Color + 'a>) -> Color;
}

pub struct SceneHit<'a> {
    pub hit: Hit,
    pub material: &'a Box<dyn Material + 'a>,
}
