mod aabb;
mod intersection;
mod tree;
mod triangle;

use super::hit::Hit;
use super::material::Material;
use super::ray::Ray;

// Scene is a collection of geometry.
pub trait Scene {
    fn intersect<'a>(&self, ray: Ray) -> (Hit, Box<dyn Material + 'a>, bool);
}
