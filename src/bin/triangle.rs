use image;
use pathtracer_rs::camera::perspective::PerspectiveCamera;
use pathtracer_rs::image_writer::Image;
use pathtracer_rs::matrix;
use pathtracer_rs::render;
use pathtracer_rs::scene;
use pathtracer_rs::scene::obj;
use pathtracer_rs::vector;
use std::f64::consts::PI;
use std::fs;

fn main() -> image::ImageResult<()> {
    let camera_matrix = matrix::IDENTITY_MATRIX
        .rotate(vector::AXIS_X, PI / -4.3)
        .translate(vector::Vector::new(0.0, 5.0, 5.0));

    let camera = PerspectiveCamera::new()
        .set_transformation_matrix(camera_matrix)
        .set_field_of_view(20.0)
        .set_depth_of_field(6.666, 0.07);

    let mut obj_file = fs::File::open("examples/triangle.obj").unwrap();
    let mut mtl_file = fs::File::open("examples/triangle.mtl").unwrap();

    let s: Box<dyn scene::Scene> = Box::new(obj::ObjScene::new(&mut obj_file, &mut mtl_file));

    let mut image_writer = Image::new(100, 100);

    let settings = render::RenderSettings {
        bounce_depth: 5,
        samples_per_ray: 50,
    };

    render::render(&s, &camera, &mut image_writer, &settings);

    let out_file = fs::File::create("examples/triangle.png").unwrap();

    image_writer.to_png8(out_file)
}
