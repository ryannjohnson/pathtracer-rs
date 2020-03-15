use super::camera::Camera;
use super::color::{Color, BLACK};
use super::material::MaterialSampler;
use super::ray::Ray;
use super::random as local_random;
use super::scene::Scene;
use crossbeam_channel;
use crossbeam_utils;
use num_cpus;
use std;

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
    camera: &'a impl Camera,
    image: &mut impl ImageWriter,
    settings: &'a RenderSettings,
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

    let samples_per_ray_reciprocal = 1.0 / settings.samples_per_ray as f64;
    let color_multiplier = Color::new(
        samples_per_ray_reciprocal,
        samples_per_ray_reciprocal,
        samples_per_ray_reciprocal,
    );

    let thread_count = num_cpus::get();
    let (tx_xy, rx_xy) = crossbeam_channel::bounded(thread_count);
    let (tx_pixel, rx_pixel) = std::sync::mpsc::channel::<PixelMessage>();
    let scene_arc = std::sync::Arc::from(scene);
    let camera_arc = std::sync::Arc::from(camera);

    crossbeam_utils::thread::scope(|scope| {
        for _ in 0..thread_count {
            let scene = std::sync::Arc::clone(&scene_arc);
            let camera = std::sync::Arc::clone(&camera_arc);
            let tx = std::sync::mpsc::Sender::clone(&tx_pixel);
            let rx = rx_xy.clone();

            scope.spawn(move |_| {
                let mut thread_random = local_random::thread::ThreadRng::new();
                let mut random: Box<dyn local_random::Rng> = Box::new(thread_random);

                for xy_message in rx {
                    match xy_message {
                        XYMessage::Done => break,
                        XYMessage::XY(x_pixel, y_pixel, x, y) => {
                            let mut color = BLACK;
                            for _ in 0..settings.samples_per_ray {
                                let x_rand =
                                    local_random::Rng::next_f64(&mut thread_random) * x_step;
                                let y_rand =
                                    local_random::Rng::next_f64(&mut thread_random) * y_step;
                                let ray = camera.cast(&mut thread_random, x + x_rand, y + y_rand);
                                let sample = sample_scene(&mut random, &scene, ray, settings.bounce_depth);
                                color = color.add(sample);
                            }
                            color = color.multiply(color_multiplier);

                            let pixel_message = PixelMessage {
                                x_pixel,
                                y_pixel,
                                color,
                            };
                            tx.send(pixel_message).unwrap();
                        }
                    }
                }
            });
        }

        scope.spawn(move |_| {
            for y_pixel in 0..height {
                // Positive is up.
                let y = y_ratio * ((y_pixel as f64 / (height - 1) as f64) - 0.5) * -1.0;
                for x_pixel in 0..width {
                    // Positive is right.
                    let x = x_ratio * (x_pixel as f64 / (width - 1) as f64 - 0.5);

                    let xy_message = XYMessage::XY(x_pixel, y_pixel, x, y);
                    tx_xy.send(xy_message).unwrap();
                }
            }

            for _ in 0..thread_count {
                tx_xy.send(XYMessage::Done).unwrap();
            }
        });

        let total_pixels = height * width;
        for _ in 0..total_pixels {
            let pixel_message = rx_pixel.recv().unwrap();
            image.set(
                pixel_message.x_pixel,
                pixel_message.y_pixel,
                pixel_message.color,
            );
        }
    })
    .unwrap();
}

enum XYMessage {
    XY(usize, usize, f64, f64),
    Done,
}

struct PixelMessage {
    pub x_pixel: usize,
    pub y_pixel: usize,
    pub color: Color,
}

fn sample_scene(random: &mut Box<dyn local_random::Rng>, scene: &Box<dyn Scene>, ray: Ray, bounce_depth: usize) -> Color {
    if bounce_depth == 0 {
        return BLACK;
    }

    let (hit, material) = match scene.intersect(ray) {
        Some(a) => a,
        None => return BLACK,
    };

    let bouncer = Box::new(Sampler {
        scene: scene,
        bounce_depth: bounce_depth - 1,
    });

    material.sample(random, hit, bouncer)
}

struct Sampler<'a> {
    scene: &'a Box<dyn Scene>,
    bounce_depth: usize,
}

impl<'a> MaterialSampler for Sampler<'a> {
    fn sample(&self, random: &mut Box<dyn local_random::Rng>, ray: Ray) -> Color {
        sample_scene(random, self.scene, ray, self.bounce_depth)
    }
}
