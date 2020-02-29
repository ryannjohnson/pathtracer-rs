use super::super::constants::EPSILON;
use super::super::ray::Ray;
use super::super::vector::Vector;
use super::intersection::Intersection;

pub trait Triangle {
    fn vertex0(&self) -> Vector;
    fn vertex1(&self) -> Vector;
    fn vertex2(&self) -> Vector;
}

/// IntersectTriangle determins if a ray passes through a triangle and at
/// what distance from the origin if so.
///
/// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution
/// http://geomalgorithms.com/a06-_intersect-2.html
pub fn intersect_triangle(ray: Ray, triangle: impl Triangle) -> Intersection {
    let mut intersection = Intersection {
        distance_from_origin: 0.0,
        position: Vector::zeros(),
        normal: Vector::zeros(),
        ok: false,
    };

    let v0v1 = triangle.vertex0().subtract(triangle.vertex1());
    let v0v2 = triangle.vertex0().subtract(triangle.vertex2());

    intersection.normal = v0v1.cross_product(v0v2).normalize();

    let cosine_of_ray_and_normal = ray.direction.dot_product(intersection.normal);

    if cosine_of_ray_and_normal.abs() < EPSILON {
        return intersection;
    }

    let v0r0 = triangle.vertex0().subtract(ray.origin);
    let min_distance_from_ray_origin_to_plane = intersection.normal.dot_product(v0r0);

    intersection.distance_from_origin =
        min_distance_from_ray_origin_to_plane / cosine_of_ray_and_normal;

    if intersection.distance_from_origin < EPSILON {
        return intersection;
    }

    intersection.position = ray
        .origin
        .add(ray.direction.scale(intersection.distance_from_origin));

    let triangleEdge = triangle.vertex1().subtract(triangle.vertex0());
    let pointEdge = intersection.position.subtract(triangle.vertex0());
    let edgesCrossProduct = triangleEdge.cross_product(pointEdge);
    if intersection.normal.dot_product(edgesCrossProduct) < 0.0 {
        return intersection;
    }

    let triangleEdge = triangle.vertex2().subtract(triangle.vertex1());
    let pointEdge = intersection.position.subtract(triangle.vertex1());
    let edgesCrossProduct = triangleEdge.cross_product(pointEdge);
    if intersection.normal.dot_product(edgesCrossProduct) < 0.0 {
        return intersection;
    }

    let triangleEdge = triangle.vertex0().subtract(triangle.vertex2());
    let pointEdge = intersection.position.subtract(triangle.vertex2());
    let edgesCrossProduct = triangleEdge.cross_product(pointEdge);
    if intersection.normal.dot_product(edgesCrossProduct) < 0.0 {
        return intersection;
    }

    intersection.ok = true;

    intersection
}
