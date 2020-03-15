mod aabb;
mod intersection;
pub mod obj;
mod tree;
mod triangle;

use super::hit::Hit;
use super::material::Material;
use super::ray::Ray;

// Scene is a collection of geometry.
pub trait Scene: Sync {
    fn intersect(&self, ray: Ray) -> Option<(Hit, &Box<dyn Material>)>;
}
