use super::super::super::ray::Ray;
use super::super::super::vector::Vector;
use super::super::aabb::AABB;
use super::super::intersection::Intersection;
use super::super::tree;
use super::super::triangle;
use std::f64::{MAX, MIN};

#[derive(Clone, Copy)]
pub struct ObjTriangle {
    pub vertexes: [Vector; 3],
    pub normals: [Vector; 3],
}

impl tree::TreeShape for ObjTriangle {
    fn aabb(&self) -> AABB {
        let mut min = Vector::new(MAX, MAX, MAX);
        let mut max = Vector::new(MIN, MIN, MIN);

        for &vertex in self.vertexes.iter() {
            min.x = min.x.min(vertex.x);
            min.y = min.y.min(vertex.y);
            min.z = min.z.min(vertex.z);
            max.x = max.x.max(vertex.x);
            max.y = max.y.max(vertex.y);
            max.z = max.z.max(vertex.z);
        }

        AABB::new(min, max)
    }

    fn intersects_aabb(&self, aabb: AABB) -> bool {
        aabb.intersects_triangle(*self)
    }

    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        triangle::intersect_triangle(ray, *self)
    }
}

impl triangle::Triangle for ObjTriangle {
    fn vertex0(&self) -> Vector {
        self.vertexes[0]
    }
    fn vertex1(&self) -> Vector {
        self.vertexes[1]
    }
    fn vertex2(&self) -> Vector {
        self.vertexes[2]
    }
}
