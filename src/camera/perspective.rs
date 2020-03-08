use super::super::matrix::{Matrix, IDENTITY_MATRIX};
use super::super::random;
use super::super::ray::Ray;
use super::super::vector;
use super::Camera;
use std::f64::consts::PI;

const AXIS_Z_MIRROR: vector::Vector = vector::Vector {
    x: 1.0,
    y: 1.0,
    z: -1.0,
};
const RADIANS_PER_DEGREE: f64 = PI / 180.0;
const TAU: f64 = PI * 2.0;

#[derive(Clone, Copy)]
pub struct PerspectiveCamera {
    depth_of_field_distance: f64, // From lens
    depth_of_field_radius: f64,
    field_of_view: f64, // Degrees
    transformation_matrix: Matrix,
}

impl PerspectiveCamera {
    pub fn new() -> PerspectiveCamera {
        PerspectiveCamera {
            depth_of_field_distance: 0.0,
            depth_of_field_radius: 0.0,
            field_of_view: 30.0,
            transformation_matrix: IDENTITY_MATRIX,
        }
    }

    pub fn set_depth_of_field(&self, distance: f64, radius: f64) -> PerspectiveCamera {
        let mut c = self.clone();
        c.depth_of_field_distance = distance;
        c.depth_of_field_radius = radius;
        c
    }

    pub fn set_field_of_view(&self, fov: f64) -> PerspectiveCamera {
        let mut c = self.clone();
        c.field_of_view = fov;
        c
    }

    pub fn set_transformation_matrix(&self, m: Matrix) -> PerspectiveCamera {
        let mut c = self.clone();
        c.transformation_matrix = m;
        c
    }
}

impl Camera for PerspectiveCamera {
    /// Cast converts the x and y coordinates into a Ray that can be cast
    /// from that point on the 2D plane.
    ///
    /// The perspective camera plots ray origins along the surface of a
    /// sphere. The size of the sphere is dictated by the magnitude of
    /// fieldOfView.
    ///
    /// When depth of field is applied, the origin of the ray will no longer
    /// originate at the x, y coordinate of the original plane but will
    /// instead be cast to intersect the focal point in front of the camera.
    ///
    /// With no transformations applied to the camera, it will point down the
    /// negative Z-axis.
    fn cast(&self, random: &mut impl random::Rng, x: f64, y: f64) -> Ray {
        let field_of_view_radians = self.field_of_view * RADIANS_PER_DEGREE;

        let m = IDENTITY_MATRIX
            .rotate(vector::AXIS_X, y * field_of_view_radians)
            .rotate(vector::AXIS_Y, x * field_of_view_radians);

        // Mirror it so that the camera looks down the negative Z-axis.
        let direction = vector::AXIS_Z.transform(m).multiply(AXIS_Z_MIRROR);

        let focal_length = 1.0 / field_of_view_radians;
        let center = vector::Vector::new(0.0, 0.0, focal_length);

        let origin = center.add(direction.scale(focal_length));

        let mut ray = Ray::new(origin, direction).transform(self.transformation_matrix);

        if self.depth_of_field_radius > 0.0 {
            let focal_origin = ray
                .origin
                .add(ray.direction.scale(self.depth_of_field_distance));

            let m = IDENTITY_MATRIX.rotate(ray.direction, TAU * random.next_f64());
            let perpendicular_axis = arbitrary_orthogonal(ray.direction).normalize().transform(m);

            let radians = self
                .depth_of_field_radius
                .atan2(self.depth_of_field_distance);
            let m = IDENTITY_MATRIX.rotate(perpendicular_axis, radians);
            let direction = ray.direction.transform(m);

            let origin = focal_origin.subtract(direction.scale(self.depth_of_field_distance));

            ray = Ray::new(origin, direction);
        }

        ray
    }
}

/// Return an arbitrary vector perpendicular to the unit vector supplied.
///
/// https://stackoverflow.com/a/43454629/5307109
fn arbitrary_orthogonal(v: vector::Vector) -> vector::Vector {
    let mut w = vector::Vector::new(0.0, 0.0, 0.0);

    if v.x < v.y && v.x < v.z {
        w.x = 1.0;
    }
    if v.y <= v.x && v.y < v.z {
        w.y = 1.0;
    }
    if v.z <= v.x && v.z <= v.y {
        w.z = 1.0;
    }

    v.cross_product(w)
}
