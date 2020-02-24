pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0 };

pub struct Color {
    pub r: f64,
    pub b: f64,
    pub g: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn add(&self, c: Color) -> Color {
        Color::new(self.r + c.r, self.g + c.g, self.b + c.b)
    }

    pub fn multiply(&self, c: Color) -> Color {
        Color::new(self.r * c.r, self.g * c.g, self.b * c.b)
    }
}
