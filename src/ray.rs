use super::matrix::Matrix;
use super::vector::Vector;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    /// Applies a transformation matrix to each of its origin and
    /// its direction, keeping the direction as a unit vector.
    pub fn transform(&self, m: Matrix) -> Ray {
        Ray::new(
            self.origin.transform(m),
            self.direction
                .transform(m.set_translation(Vector::new(0.0, 0.0, 0.0)))
                .normalize(),
        )
    }
}
