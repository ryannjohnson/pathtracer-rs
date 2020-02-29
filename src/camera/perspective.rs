use super::super::constants::EPSILON;
use super::super::matrix::{Matrix, IDENTITY_MATRIX};
use super::super::random;
use super::super::ray::Ray;
use super::super::vector;
use super::Camera;
use std::f64::consts::{FRAC_PI_2, PI};

const TAU: f64 = PI * 2.0;

pub struct Perspective {
    depth_of_field_distance: f64,
    depth_of_field_radius: f64,
    field_of_view: f64,
    transformation_matrix: Matrix,
}

impl Perspective {
    pub fn new() -> Perspective {
        Perspective {
            depth_of_field_distance: 0.0,
            depth_of_field_radius: 0.0,
            field_of_view: 30.0,
            transformation_matrix: IDENTITY_MATRIX,
        }
    }

    pub fn set_depth_of_field(&mut self, distance: f64, radius: f64) {
        self.depth_of_field_distance = distance;
        self.depth_of_field_radius = radius;
    }

    pub fn set_field_of_view(&mut self, fov: f64) {
        self.field_of_view = fov;
    }

    pub fn set_transformation_matrix(&mut self, m: Matrix) {
        self.transformation_matrix = m;
    }
}

impl Camera for Perspective {
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
    fn cast(&self, random: &mut impl random::Rng, x: f64, y: f64) -> Ray {
        let field_of_view_radians = self.field_of_view * FRAC_PI_2;

        let m = IDENTITY_MATRIX
            .rotate(vector::AXIS_X, y * field_of_view_radians)
            .rotate(vector::AXIS_Y, x * field_of_view_radians);

        let direction = vector::AXIS_Z.transform(m);

        let focal_length = 1.0 / field_of_view_radians;
        let center = vector::Vector::new(0.0, 0.0, -focal_length);

        let origin = center.add(direction.scale(focal_length));

        let mut ray = Ray::new(origin, direction).transform(self.transformation_matrix);

        if self.depth_of_field_radius >= EPSILON {
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
