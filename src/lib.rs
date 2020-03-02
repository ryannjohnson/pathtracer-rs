pub mod camera;
mod color;
mod constants;
mod hit;
pub mod image_writer;
mod material;
pub mod matrix;
mod random;
mod ray;
pub mod render;
pub mod scene;
pub mod vector;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
