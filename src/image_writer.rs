use super::color::{Color, BLACK};
use super::render::ImageWriter;
use image;
use image::png;
use std::io;

pub struct Image {
    height: usize,
    colors: Vec<Color>,
    width: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            height,
            colors: vec![BLACK; width * height],
            width,
        }
    }

    pub fn to_png8(&self, writer: impl io::Write) -> image::ImageResult<()> {
        let mut data = vec![0; self.width * self.height * 3];

        for (i, &color) in self.colors.iter().enumerate() {
            data[i * 3] = to_u8(color.r);
            data[i * 3 + 1] = to_u8(color.g);
            data[i * 3 + 2] = to_u8(color.b);
        }

        png::PNGEncoder::new(writer).encode(
            &data,
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgb8,
        )
    }
}

impl ImageWriter for Image {
    fn height(&self) -> usize {
        self.height
    }

    fn set(&mut self, x: usize, y: usize, color: Color) {
        let index = self.height * y + x;

        if index >= self.colors.len() {
            panic!("pixel index is out of range")
        }

        self.colors[index] = color;
    }

    fn width(&self) -> usize {
        self.width
    }
}

fn to_u8(v: f64) -> u8 {
    (v * 255.0) as u8
}
