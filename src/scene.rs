use super::hit::Hit;
use super::material::Material;
use super::ray::Ray;

// Scene is a collection of geometry.
trait Scene {
    fn intersect<M: Material>(ray: Ray) -> (Hit, Box<M>, bool);
}
