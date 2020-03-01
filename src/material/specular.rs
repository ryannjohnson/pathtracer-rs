use super::super::vector::Vector;

// Reflects the incident vector across the normal.
//
// https://en.wikipedia.org/wiki/Specular_reflection
pub fn bounce(normal: Vector, incident: Vector) -> Vector {
    let incident_height = -incident.dot_product(normal);

    let scaled_normal = normal.scale(2.0 * incident_height);

    scaled_normal.add(incident)
}
