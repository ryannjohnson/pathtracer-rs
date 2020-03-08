use super::matrix::Matrix;

pub const AXIS_X: Vector = Vector {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
pub const AXIS_Y: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub const AXIS_Z: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn zeros() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    pub fn add(&self, v: Vector) -> Vector {
        Vector::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }

    /// Measures the magnitude of similarity between two vectors.
    ///
    /// If they're parallel, then a vector representing the product of their
    /// lengths is returned.
    ///
    /// If they're perpendicular, then zero is returned.
    ///
    /// https://en.wikipedia.org/wiki/Dot_product
    pub fn dot_product(&self, v: Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// Measures the similarity between two vectors.
    ///
    /// If they are parallel, the cross product is zero because there is no
    /// difference between them.
    ///
    /// If they are perpendicular, the cross product will equal a vector with
    /// a length equal to the area of the triangle created by the two
    /// vectors.
    ///
    /// Eg, if v1 = (1, 0, 0) and v2 = (0, 1, 0), then the cross product will
    /// be (0, 0, 1).
    ///
    /// https://en.wikipedia.org/wiki/Cross_product
    pub fn cross_product(&self, v: Vector) -> Vector {
        Vector::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn multiply(&self, v: Vector) -> Vector {
        Vector::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }

    pub fn normalize(&self) -> Vector {
        let length = self.length();
        Vector::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn scale(&self, multiplier: f64) -> Vector {
        Vector::new(
            self.x * multiplier,
            self.y * multiplier,
            self.z * multiplier,
        )
    }

    pub fn subtract(&self, v: Vector) -> Vector {
        Vector::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }

    pub fn transform(&self, m: Matrix) -> Vector {
        Vector::new(
            self.x * m.x00 + self.y * m.x01 + self.z * m.x02 + m.x03,
            self.x * m.x10 + self.y * m.x11 + self.z * m.x12 + m.x13,
            self.x * m.x20 + self.y * m.x21 + self.z * m.x22 + m.x23,
        )
    }
}
