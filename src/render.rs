use super::camera::Camera;
use super::color::{Color, BLACK};
use super::random as local_random;
use super::ray::Ray;
use super::scene::Scene;

pub struct RenderSettings {
    pub bounce_depth: usize,
    pub samples_per_ray: usize,
}

pub trait ImageWriter {
    fn height(&self) -> usize;
    fn set(&self, x: usize, y: usize, color: Color);
    fn width(&self) -> usize;
}

pub fn render<'a>(
    scene: &'a Box<dyn Scene>,
    camera: &impl Camera,
    image: &impl ImageWriter,
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

    let random = &mut local_random::thread::ThreadRng::new();
    let mut random_box: Box<dyn local_random::Rng> = Box::new(random.clone());

    let scene_content = SceneContent {
        random: &mut random_box,
        scene: scene,
    };

    for y_pixel in 0..height {
        let y = y_ratio * ((y_pixel as f64 / (height - 1) as f64) - 0.5) * -1.0;
        for x_pixel in 0..width {
            let x = x_ratio * (x_pixel as f64 / (width - 1) as f64 - 0.5);

            let mut colors: Vec<Color> = Vec::with_capacity(settings.samples_per_ray);
            for i in 0..settings.samples_per_ray {
                let x_rand = local_random::Rng::next_f64(random) * x_step;
                let y_rand = local_random::Rng::next_f64(random) * y_step;
                let ray = camera.cast(random, x + x_rand, y + y_rand);
                colors[i] = scene_content.sample(ray, settings.bounce_depth);
            }
            let color = average_of(colors);

            image.set(x_pixel, y_pixel, color);
        }
    }
}

struct SceneContent<'a> {
    random: &'a mut Box<dyn local_random::Rng>,
    scene: &'a Box<dyn Scene>,
}

impl<'a> SceneContent<'a> {
    pub fn sample(&self, ray: Ray, bounces_left: usize) -> Color {
        if bounces_left == 0 {
            return BLACK;
        }

        self.scene.intersect(
            ray,
            Box::new(|scene_hit| {
                (*scene_hit.material).sample(
                    &mut self.random,
                    scene_hit.hit,
                    Box::new(|next_ray| self.sample(next_ray, bounces_left - 1)),
                )
            }),
        )
    }
}

fn average_of(colors: Vec<Color>) -> Color {
    let mut output = Color::new(0.0, 0.0, 0.0);
    let total = colors.len();

    for &color in colors.iter() {
        output = output.add(color);
    }

    let total_float = total as f64;
    output.r = output.r / total_float;
    output.g = output.g / total_float;
    output.b = output.b / total_float;

    output
}
