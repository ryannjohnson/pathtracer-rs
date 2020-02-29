use super::super::vector::Vector;

pub struct Intersection {
    pub distance_from_origin: f64,
    pub position: Vector,
    pub normal: Vector,
    pub ok: bool,
}
