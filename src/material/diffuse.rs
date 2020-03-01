use super::super::constants::EPSILON;
use super::super::random::Rng;
use super::super::vector::Vector;

// Returns a unit vector in the hemisphere of the supplied normal.
pub fn bounce(random: &mut Box<dyn Rng>, normal: Vector) -> Vector {
    loop {
        let vector = Vector::new(
            random.next_f64() * 2.0 - 1.0,
            random.next_f64() * 2.0 - 1.0,
            random.next_f64() * 2.0 - 1.0,
        );

        let vector_length = vector.length();

        if vector_length >= 1.0 || vector_length < EPSILON {
            continue;
        }

        return vector.scale(vector.dot_product(normal)).normalize();
    }
}
