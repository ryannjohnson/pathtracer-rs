use super::super::super::color::{Color, BLACK};
use super::super::super::hit::Hit;
use super::super::super::material::{Material, MaterialSampler};
use super::super::super::random::Rng;
use super::super::super::ray::Ray;
use super::super::super::vector::Vector;
use super::super::aabb::AABB;
use super::super::tree;
use super::super::Scene;
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

        let mut i = 0;

        for object in obj_set.objects.iter() {
            for geometry in object.geometry.iter() {
                let material_name = geometry.material_name.clone().unwrap();
                let material_index = material_indexes.get(&material_name).unwrap();

                for shape in geometry.shapes.iter() {
                    match shape.primitive {
                        wavefront_obj::obj::Primitive::Point(_) => continue,
                        wavefront_obj::obj::Primitive::Line(_, _) => continue,
                        wavefront_obj::obj::Primitive::Triangle(v0, v1, v2) => {
                            let vertexes = [
                                Vector::new(
                                    object.vertices[v0.0].x,
                                    object.vertices[v0.0].y,
                                    object.vertices[v0.0].z,
                                ),
                                Vector::new(
                                    object.vertices[v1.0].x,
                                    object.vertices[v1.0].y,
                                    object.vertices[v1.0].z,
                                ),
                                Vector::new(
                                    object.vertices[v2.0].x,
                                    object.vertices[v2.0].y,
                                    object.vertices[v2.0].z,
                                ),
                            ];

                            let normals: [Vector; 3];

                            if v0.2.is_some() && v1.2.is_some() && v2.2.is_some() {
                                normals = [
                                    Vector::new(
                                        object.vertices[v0.2.unwrap()].x,
                                        object.vertices[v0.2.unwrap()].y,
                                        object.vertices[v0.2.unwrap()].z,
                                    ),
                                    Vector::new(
                                        object.vertices[v1.2.unwrap()].x,
                                        object.vertices[v1.2.unwrap()].y,
                                        object.vertices[v1.2.unwrap()].z,
                                    ),
                                    Vector::new(
                                        object.vertices[v2.2.unwrap()].x,
                                        object.vertices[v2.2.unwrap()].y,
                                        object.vertices[v2.2.unwrap()].z,
                                    ),
                                ];
                            } else {
                                let v0v1 = vertexes[0].subtract(vertexes[1]);
                                let v0v2 = vertexes[0].subtract(vertexes[2]);
                                let normal = v0v1.cross_product(v0v2).normalize();
                                normals = [normal, normal, normal];
                            }

                            let obj_triangle = ObjTriangle { normals, vertexes };

                            tree_shapes.push(Box::new(obj_triangle));
                            tree_shape_indexes.push(i);
                            i += 1;
                            tree_shape_material_indexes.push(*material_index);

                            for &vertex in obj_triangle.vertexes.iter() {
                                aabb_min.x = aabb_min.x.min(vertex.x);
                                aabb_min.y = aabb_min.y.min(vertex.y);
                                aabb_min.z = aabb_min.z.min(vertex.z);
                                aabb_max.x = aabb_max.x.max(vertex.x);
                                aabb_max.y = aabb_max.y.max(vertex.y);
                                aabb_max.z = aabb_max.z.max(vertex.z);
                            }
                        }
                    }
                }
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

    fn intersect(&self, ray: Ray) -> Option<(Hit, &Box<dyn Material>)> {
        tree::intersect_tree_node(&self.tree_shapes, &self.tree, ray).map(|intersection| {
            let material_index = self.tree_shape_material_indexes[intersection.nearest_shape_index];
            (intersection.hit, &self.materials[material_index])
        })
    }
}

impl Scene for ObjScene {
    fn sample(&self, random: &mut Box<dyn Rng>, ray: Ray, bounce_depth: usize) -> Color {
        if bounce_depth == 0 {
            return BLACK;
        }

        let (hit, material) = match self.intersect(ray) {
            Some(a) => a,
            None => return BLACK,
        };

        let bouncer = Box::new(ObjSceneSampler {
            scene: self,
            bounce_depth: bounce_depth - 1,
        });

        material.sample(random, hit, bouncer)
    }
}

struct ObjSceneSampler<'a> {
    scene: &'a ObjScene,
    bounce_depth: usize,
}

impl<'a> MaterialSampler for ObjSceneSampler<'a> {
    fn sample(&self, random: &mut Box<dyn Rng>, ray: Ray) -> Color {
        self.scene.sample(random, ray, self.bounce_depth)
    }
}
