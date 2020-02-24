use super::vector::Vector;

const IDENTITY_MATRIX: Matrix = Matrix {
    x00: 1.0,
    x01: 0.0,
    x02: 0.0,
    x03: 0.0,
    x10: 0.0,
    x11: 1.0,
    x12: 0.0,
    x13: 0.0,
    x20: 0.0,
    x21: 0.0,
    x22: 1.0,
    x23: 0.0,
    x30: 0.0,
    x31: 0.0,
    x32: 0.0,
    x33: 1.0,
};

#[derive(Copy, Clone)]
pub struct Matrix {
    pub x00: f64,
    pub x01: f64,
    pub x02: f64,
    pub x03: f64,
    pub x10: f64,
    pub x11: f64,
    pub x12: f64,
    pub x13: f64,
    pub x20: f64,
    pub x21: f64,
    pub x22: f64,
    pub x23: f64,
    pub x30: f64,
    pub x31: f64,
    pub x32: f64,
    pub x33: f64,
}

impl Matrix {
    pub fn new(
        x00: f64,
        x01: f64,
        x02: f64,
        x03: f64,
        x10: f64,
        x11: f64,
        x12: f64,
        x13: f64,
        x20: f64,
        x21: f64,
        x22: f64,
        x23: f64,
        x30: f64,
        x31: f64,
        x32: f64,
        x33: f64,
    ) -> Matrix {
        Matrix {
            x00,
            x01,
            x02,
            x03,
            x10,
            x11,
            x12,
            x13,
            x20,
            x21,
            x22,
            x23,
            x30,
            x31,
            x32,
            x33,
        }
    }

    pub fn set_translation(&self, v: Vector) -> Matrix {
        let mut m = *self;
        m.x03 = v.x;
        m.x13 = v.y;
        m.x23 = v.z;
        m
    }
}
