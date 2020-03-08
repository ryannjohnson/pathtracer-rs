use super::camera::Camera;
use super::color::{Color, BLACK};
use super::random as local_random;
use super::scene::Scene;

pub struct RenderSettings {
    pub bounce_depth: usize,
    pub samples_per_ray: usize,
}

pub trait ImageWriter {
    fn height(&self) -> usize;
    fn set(&mut self, x: usize, y: usize, color: Color);
    fn width(&self) -> usize;
}

pub fn render<'a>(
    scene: &'a Box<dyn Scene>,
    camera: &impl Camera,
    image: &mut impl ImageWriter,
    settings: &RenderSettings,
) {
    let height = image.height();
    let width = image.width();

    let aspect_ratio = width as f64 / height as f64;

    let mut x_ratio: f64 = 1.0;
    let mut y_ratio: f64 = 1.0;
    if aspect_ratio < 1.0 {
        x_ratio = aspect_ratio;
    } else {
        y_ratio = 1.0 / aspect_ratio;
    }

    let x_step = x_ratio / (width - 1) as f64;
    let y_step = y_ratio / (height - 1) as f64;

    let mut thread_random = local_random::thread::ThreadRng::new();
    let mut random: Box<dyn local_random::Rng> = Box::new(thread_random);

    let samples_per_ray_reciprocal = 1.0 / settings.samples_per_ray as f64;
    let color_multiplier = Color::new(
        samples_per_ray_reciprocal,
        samples_per_ray_reciprocal,
        samples_per_ray_reciprocal,
    );

    for y_pixel in 0..height {
        let y = y_ratio * ((y_pixel as f64 / (height - 1) as f64) - 0.5) * -1.0;
        for x_pixel in 0..width {
            let x = x_ratio * (x_pixel as f64 / (width - 1) as f64 - 0.5);

            let mut color = BLACK;
            for _ in 0..settings.samples_per_ray {
                let x_rand = local_random::Rng::next_f64(&mut thread_random) * x_step;
                let y_rand = local_random::Rng::next_f64(&mut thread_random) * y_step;
                let ray = camera.cast(&mut thread_random, x + x_rand, y + y_rand);
                let sample = scene.sample(&mut random, ray, settings.bounce_depth);
                color = color.add(sample);
            }

            color = color.multiply(color_multiplier);
            image.set(x_pixel, y_pixel, color);
        }
    }
}
