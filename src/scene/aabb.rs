use super::super::ray::Ray;
use super::super::vector::{Vector, AXIS_X, AXIS_Y, AXIS_Z};
use super::triangle::Triangle;
use std::f64::{MAX, MIN};

#[derive(Clone, Copy)]
pub struct AABB {
    min: Vector,
    max: Vector,
}

pub struct AABBRayIntersection {
    tmin: f64,
    tmax: f64,
    ok: bool,
}

impl AABBRayIntersection {
    pub fn tmin(&self) -> f64 {
        self.tmin
    }

    pub fn tmax(&self) -> f64 {
        self.tmax
    }

    pub fn ok(&self) -> bool {
        self.ok
    }
}

impl AABB {
    pub fn new(min: Vector, max: Vector) -> AABB {
        AABB { min, max }
    }

    pub fn min(&self) -> Vector {
        self.min
    }

    pub fn max(&self) -> Vector {
        self.max
    }

    pub fn vertexes(&self) -> Vec<Vector> {
        vec![
            Vector::new(self.min.x, self.min.y, self.min.z),
            Vector::new(self.max.x, self.min.y, self.min.z),
            Vector::new(self.min.x, self.max.y, self.min.z),
            Vector::new(self.min.x, self.min.y, self.max.z),
            Vector::new(self.min.x, self.max.y, self.max.z),
            Vector::new(self.max.x, self.min.y, self.max.z),
            Vector::new(self.max.x, self.max.y, self.min.z),
            Vector::new(self.max.x, self.max.y, self.max.z),
        ]
    }

    /// IntersectsRay determines whether a ray intersects a box at all. This
    /// includes if the ray's origin is inside the box.
    ///
    /// https://gamedev.stackexchange.com/a/18459
    pub fn intersects_ray(&self, ray: Ray) -> AABBRayIntersection {
        let tx0 = (self.min.x - ray.origin.x) / ray.direction.x;
        let tx1 = (self.max.x - ray.origin.x) / ray.direction.x;
        let ty0 = (self.min.y - ray.origin.y) / ray.direction.y;
        let ty1 = (self.max.y - ray.origin.y) / ray.direction.y;
        let tz0 = (self.min.z - ray.origin.z) / ray.direction.z;
        let tz1 = (self.max.z - ray.origin.z) / ray.direction.z;

        let tmin = tx0.min(tx1).max(ty0.min(ty1)).max(tz0.min(tz1));
        let tmax = tx0.max(tx1).min(ty0.max(ty1)).min(tz0.max(tz1));

        AABBRayIntersection {
            tmin,
            tmax,
            ok: tmin <= tmax,
        }
    }

    pub fn intersects_triangle(&self, triangle: impl Triangle) -> bool {
        let aabb_edges: [Vector; 3] = [AXIS_X, AXIS_Y, AXIS_Z];
        let triangle_vertexes = &vec![triangle.vertex0(), triangle.vertex1(), triangle.vertex2()];

        let mut triangle_t: TminTmax;

        triangle_t = project_distance_along_axis(triangle_vertexes, AXIS_X);
        if triangle_t.min > self.max.x || triangle_t.max < self.min.x {
            return false;
        }
        triangle_t = project_distance_along_axis(triangle_vertexes, AXIS_Y);
        if triangle_t.min > self.max.y || triangle_t.max < self.min.y {
            return false;
        }
        triangle_t = project_distance_along_axis(triangle_vertexes, AXIS_Z);
        if triangle_t.min > self.max.z || triangle_t.max < self.min.z {
            return false;
        }

        let v0v1 = triangle.vertex0().subtract(triangle.vertex1());
        let v0v2 = triangle.vertex0().subtract(triangle.vertex2());
        let triangle_normal = v0v1.cross_product(v0v2);
        let triangle_distance_from_origin = triangle_normal.dot_product(triangle.vertex0());
        let aabb_vertexes = &self.vertexes();

        triangle_t = project_distance_along_axis(aabb_vertexes, triangle_normal);
        if triangle_t.min > triangle_distance_from_origin
            || triangle_t.max < triangle_distance_from_origin
        {
            return false;
        }

        let v1v2 = triangle.vertex1().subtract(triangle.vertex2());
        for triangle_edge in vec![v0v1, v1v2, v1v2].iter() {
            for aabb_edge in aabb_edges.iter() {
                let axis = triangle_edge.cross_product(*aabb_edge);
                triangle_t = project_distance_along_axis(triangle_vertexes, axis);
                let aabb_t = project_distance_along_axis(aabb_vertexes, axis);

                if aabb_t.min > triangle_t.max || aabb_t.max < triangle_t.min {
                    return false;
                }
            }
        }

        true
    }
}

struct TminTmax {
    pub min: f64,
    pub max: f64,
}

/// projectDistanceAlongAxis determines the closest and farthest
/// distances along an arbitrary axis that any of the included vectors
/// reach. Distances are all relative to the origin.
fn project_distance_along_axis(vertexes: &Vec<Vector>, axis: Vector) -> TminTmax {
    let mut min = MAX;
    let mut max = MIN;
    let mut distance: f64;

    for vertex in vertexes.iter() {
        distance = vertex.dot_product(axis);

        if min > distance {
            min = distance;
        }

        if max < distance {
            max = distance;
        }
    }

    TminTmax { min, max }
}
