use super::ray::Ray;
use super::vector::Vector;

pub struct Hit {
    pub from: Ray,
    pub position: Vector,
    pub normal: Vector,
    // pub uv: UVCoordinate,
}

pub struct UVCoordinate {
    x: f64,
    y: f64,
}
