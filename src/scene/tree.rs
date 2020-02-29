use super::super::constants::EPSILON;
use super::super::hit::Hit;
use super::super::ray::Ray;
use super::super::vector::Vector;
use super::aabb::AABB;
use super::intersection::Intersection;
use std::f64::MAX;

/// Anything that occupies 3D space. It can be indexed in an
/// AABB (axis-aligned bounding box) tree and can be intersected by rays.
pub trait TreeShape {
    fn aabb(&self) -> AABB;
    fn intersects_aabb(&self, aabb: AABB) -> bool;
    fn intersect_ray(&self, ray: Ray) -> Intersection;
}

pub struct TreeNode {
    aabb: AABB,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    shape_indexes: Vec<usize>,
}

/// Constructs a tree of TreeNodes that can be queried for shape
/// indexes.
///
/// The root TreeNode should contain a box that encapsulates all the
/// shapes in the scene. That box gets subdivided along its longest axis
/// until it either contains very few shapes or is notably smaller than
/// any of the shapes it contains. This frequently happens when the box
/// encapsulates a corner of 3 or more shapes.
///
/// TreeNodes can either be branches or leaves. Branches have zero shape
/// indexes and always have both their `left` and `right` nodes. Leaves
/// always have a nonzero number of shape indexes and neither of their
/// `left` or `right` nodes.
pub fn build_tree_node(
    shapes: &Vec<Box<dyn TreeShape>>,
    possible_shape_indexes: &Vec<usize>,
    aabb: AABB,
) -> Option<Box<TreeNode>> {
    let mut shape_indexes: Vec<usize> = vec![];
    for &shape_index in possible_shape_indexes.iter() {
        let shape = &shapes[shape_index];
        if shape.intersects_aabb(aabb) {
            shape_indexes.push(shape_index);
        }
    }

    if shape_indexes.len() == 0 {
        return None;
    }

    if shape_indexes.len() <= 3 {
        return Option::from(Box::new(TreeNode {
            aabb,
            left: None,
            right: None,
            shape_indexes,
        }));
    }

    let mut min_shape_length = MAX;
    for &shape_index in shape_indexes.iter() {
        let shape = &shapes[shape_index];
        let shape_aabb = shape.aabb();
        let shape_length = shape_aabb.max().subtract(shape_aabb.min()).length();

        if min_shape_length > shape_length {
            min_shape_length = shape_length;
        }
    }

    let max_aabb_length = aabb.max().subtract(aabb.min()).length() * 4.0;
    if max_aabb_length < min_shape_length || min_shape_length < EPSILON {
        return Option::from(Box::new(TreeNode {
            aabb,
            left: None,
            right: None,
            shape_indexes,
        }));
    }

    let a_and_b = split_by_longest_axis(aabb);

    // TODO: Parallelize.
    let left = build_tree_node(shapes, &shape_indexes, a_and_b.0);
    let right = build_tree_node(shapes, &shape_indexes, a_and_b.1);

    if left.is_some() && right.is_some() {
        return Option::from(Box::new(TreeNode {
            aabb,
            left,
            right,
            shape_indexes: vec![],
        }));
    }

    if left.is_some() {
        return left;
    }

    if right.is_some() {
        return right;
    }

    panic!("shapes were dropped in the tree building process");
}

pub struct TreeNodeIntersection {
    hit: Hit,
    nearest_shape_index: usize,
}

/// IntersectTreeNode searches the node tree, intersecting each node in
/// the order according to the ray's trajectory, trying to intersect each
/// shape until its first hit.
///
/// NOTE: It turns out to be more performant to pass in []TreeShape
/// instead of a getter interface.
pub fn intersect_tree_node(
    shapes: &Vec<Box<dyn TreeShape>>,
    tree_node: &Box<TreeNode>,
    ray: Ray,
) -> Option<TreeNodeIntersection> {
    if tree_node.left.is_some() && tree_node.right.is_some() {
        let left_intersection = tree_node.left.as_ref().unwrap().aabb.intersects_ray(ray);
        let right_intersection = tree_node.right.as_ref().unwrap().aabb.intersects_ray(ray);

        if left_intersection.ok() && right_intersection.ok() {
            if left_intersection.tmin() < right_intersection.tmin() {
                let tree_node_intersection =
                    intersect_tree_node(shapes, tree_node.left.as_ref().unwrap(), ray);
                if tree_node_intersection.is_some() {
                    return tree_node_intersection;
                }
                return intersect_tree_node(shapes, tree_node.right.as_ref().unwrap(), ray);
            }
            let tree_node_intersection =
                intersect_tree_node(shapes, tree_node.right.as_ref().unwrap(), ray);
            if tree_node_intersection.is_some() {
                return tree_node_intersection;
            }
            return intersect_tree_node(shapes, tree_node.left.as_ref().unwrap(), ray);
        }

        if left_intersection.ok() {
            return intersect_tree_node(shapes, tree_node.left.as_ref().unwrap(), ray);
        }

        if right_intersection.ok() {
            return intersect_tree_node(shapes, tree_node.right.as_ref().unwrap(), ray);
        }

        return None;
    }

    let mut nearest_position = Vector::zeros();
    let mut nearest_normal = Vector::zeros();
    let mut nearest_distance = MAX;
    let mut nearest_shape_index: usize = 0;

    let aabb_intersection = tree_node.aabb.intersects_ray(ray);

    for &shape_index in tree_node.shape_indexes.iter() {
        let shape = &shapes[shape_index];
        let shape_intersection = shape.intersect_ray(ray);
        if !shape_intersection.ok {
            continue;
        }

        if shape_intersection.distance_from_origin < aabb_intersection.tmin()
            || shape_intersection.distance_from_origin > aabb_intersection.tmax()
        {
            continue;
        }

        if nearest_distance < shape_intersection.distance_from_origin {
            continue;
        }

        nearest_position = shape_intersection.position;
        nearest_normal = shape_intersection.normal;
        nearest_distance = shape_intersection.distance_from_origin;
        nearest_shape_index = shape_index;
    }

    if nearest_distance == MAX {
        return None;
    }

    Option::from(TreeNodeIntersection {
        hit: Hit {
            from: ray,
            position: nearest_position,
            normal: nearest_normal,
        },
        nearest_shape_index,
    })
}

fn split_by_longest_axis(aabb: AABB) -> (AABB, AABB) {
    let d = aabb.max().subtract(aabb.min());
    let middle: f64;
    let a0: Vector;
    let a1: Vector;
    let b0: Vector;
    let b1: Vector;
    let a: AABB;
    let b: AABB;

    if d.x >= d.y && d.x >= d.z {
        // X is longest.
        middle = aabb.min().x + d.x / 2.0;

        a0 = aabb.min();
        a1 = Vector::new(middle, aabb.max().y, aabb.max().z);
        a = AABB::new(a0, a1);

        b0 = Vector::new(middle, aabb.min().y, aabb.min().z);
        b1 = aabb.max();
        b = AABB::new(b0, b1);
        return (a, b);
    }

    if d.y >= d.x && d.y >= d.z {
        // Y is longest.
        middle = aabb.min().y + d.y / 2.0;

        a0 = aabb.min();
        a1 = Vector::new(aabb.max().x, middle, aabb.max().z);
        a = AABB::new(a0, a1);

        b0 = Vector::new(aabb.min().x, middle, aabb.min().z);
        b1 = aabb.max();
        b = AABB::new(b0, b1);
        return (a, b);
    }

    // Z is longest.
    middle = aabb.min().z + d.z / 2.0;

    a0 = aabb.min();
    a1 = Vector::new(aabb.max().x, aabb.max().y, middle);
    a = AABB::new(a0, a1);

    b0 = Vector::new(aabb.min().x, aabb.min().y, middle);
    b1 = aabb.max();
    b = AABB::new(b0, b1);
    (a, b)
}
