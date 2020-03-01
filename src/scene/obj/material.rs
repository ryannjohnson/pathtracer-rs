use super::super::super::color::{Color, BLACK};
use super::super::super::hit::Hit;
use super::super::super::material::{Material, MaterialSampler};
use super::super::super::material::{diffuse, specular};
use super::super::super::random::Rng;
use super::super::super::ray::Ray;
use wavefront_obj::mtl;

pub struct ObjMaterial {
    source: mtl::Material,
}

impl ObjMaterial {
    pub fn new(source: mtl::Material) -> ObjMaterial {
        ObjMaterial { source }
    }
}

impl Material for ObjMaterial {
    fn sample<'a>(
        &self,
        random: &'a mut Box<dyn Rng>,
        hit: Hit,
        sampler: Box<dyn MaterialSampler + 'a>,
    ) -> Color {
        let mut color = BLACK;

        if self.source.color_diffuse.r > 0.0
            || self.source.color_diffuse.g > 0.0
            || self.source.color_diffuse.b > 0.0
        {
            let ray = Ray {
                origin: hit.position,
                direction: diffuse::bounce(random, hit.normal),
            };

            let color_from_scene = sampler.sample(random, ray);
            let color_to_camera = color_from_scene.multiply(to_color(self.source.color_diffuse));

            color = color.add(color_to_camera);
        }

        if self.source.color_specular.r > 0.0
            || self.source.color_specular.g > 0.0
            || self.source.color_specular.b > 0.0
        {
            let ray = Ray {
                origin: hit.position,
                direction: specular::bounce(hit.normal, hit.from.direction),
            };

            let color_from_scene = sampler.sample(random, ray);
            let color_to_camera = color_from_scene.multiply(to_color(self.source.color_diffuse));

            color = color.add(color_to_camera);
        }

        if self.source.color_emissive.is_some() {
            color.add(to_color(self.source.color_emissive.unwrap()));
        }

        color
    }
}

fn to_color(color: mtl::Color) -> Color {
    Color::new(color.r, color.g, color.b)
}
