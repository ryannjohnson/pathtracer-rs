use super::super::super::color::{Color, BLACK};
use super::super::super::material::Material;
use super::super::super::ray::Ray;
use super::super::super::vector::Vector;
use super::super::aabb::AABB;
use super::super::tree;
use super::super::{Scene, SceneHit};
use super::material::ObjMaterial;
use super::triangle::ObjTriangle;
use std::collections;
use std::f64::{MAX, MIN};
use std::io;
use wavefront_obj;

pub struct ObjScene {
    materials: Vec<Box<dyn Material>>,
    tree: Box<tree::TreeNode>,
    tree_shapes: Vec<Box<dyn tree::TreeShape>>,
    tree_shape_material_indexes: Vec<usize>,
}

impl ObjScene {
    pub fn new(obj: &mut impl io::Read, mtl: &mut impl io::Read) -> ObjScene {
        let mut obj_string = String::new();
        let mut mtl_string = String::new();

        obj.read_to_string(&mut obj_string).unwrap();
        let obj_set = wavefront_obj::obj::parse(obj_string).unwrap();

        mtl.read_to_string(&mut mtl_string).unwrap();
        let mtl_set = wavefront_obj::mtl::parse(mtl_string).unwrap();

        let mut materials: Vec<Box<dyn Material>> = vec![];
        let mut material_indexes = collections::HashMap::new();
        for (i, material) in mtl_set.materials.iter().enumerate() {
            materials.push(Box::new(ObjMaterial::new(material.clone())));
            material_indexes.insert(material.name.clone(), i);
        }

        let mut tree_shapes: Vec<Box<dyn tree::TreeShape>> = vec![];
        let mut tree_shape_indexes = vec![];
        let mut tree_shape_material_indexes = vec![];
        let mut aabb_min = Vector::new(MAX, MAX, MAX);
        let mut aabb_max = Vector::new(MIN, MIN, MIN);

        for (i, object) in obj_set.objects.iter().enumerate() {
            let vertexes = [
                Vector::new(
                    object.vertices[0].x,
                    object.vertices[0].y,
                    object.vertices[0].z,
                ),
                Vector::new(
                    object.vertices[1].x,
                    object.vertices[1].y,
                    object.vertices[1].z,
                ),
                Vector::new(
                    object.vertices[2].x,
                    object.vertices[2].y,
                    object.vertices[2].z,
                ),
            ];

            let normals = [
                Vector::new(
                    object.normals[0].x,
                    object.normals[0].y,
                    object.normals[0].z,
                ),
                Vector::new(
                    object.normals[1].x,
                    object.normals[1].y,
                    object.normals[1].z,
                ),
                Vector::new(
                    object.normals[2].x,
                    object.normals[2].y,
                    object.normals[2].z,
                ),
            ];

            if object.geometry.len() == 0 {
                panic!("object has no geometry");
            }

            let material_name = object.geometry[0].material_name.clone().unwrap();
            let material_index = material_indexes.get(&material_name).unwrap();

            let t = ObjTriangle { normals, vertexes };

            tree_shapes.push(Box::new(t));
            tree_shape_indexes.push(i);
            tree_shape_material_indexes.push(*material_index);

            for &vertex in t.vertexes.iter() {
                aabb_min.x = aabb_min.x.min(vertex.x);
                aabb_min.y = aabb_min.y.min(vertex.y);
                aabb_min.z = aabb_min.z.min(vertex.z);
                aabb_max.x = aabb_max.x.max(vertex.x);
                aabb_max.y = aabb_max.y.max(vertex.y);
                aabb_max.z = aabb_max.z.max(vertex.z);
            }
        }

        let root_aabb = AABB::new(aabb_min, aabb_max);

        ObjScene {
            materials,
            tree: tree::build_tree_node(&tree_shapes, &tree_shape_indexes, root_aabb).unwrap(),
            tree_shapes,
            tree_shape_material_indexes,
        }
    }
}

impl Scene for ObjScene {
    fn intersect<'a>(&self, ray: Ray, callback: Box<dyn Fn(SceneHit) -> Color + 'a>) -> Color {
        let color_option =
            tree::intersect_tree_node(&self.tree_shapes, &self.tree, ray).map(|intersection| {
                let material_index =
                    self.tree_shape_material_indexes[intersection.nearest_shape_index];

                callback(SceneHit {
                    hit: intersection.hit,
                    material: &self.materials[material_index],
                })
            });

        match color_option {
            Some(a) => a,
            None => BLACK,
        }
    }
}
