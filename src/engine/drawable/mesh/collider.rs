use glam::{Vec3, Vec4};
use crate::engine::transform::Transform;

//TODO: USE TRANSFORM NOT CENTER
pub trait ColliderArea {
    fn in_frustum(&self, transform: &Transform, frustum: &[Vec4; 6]) -> bool;
}

pub struct ColliderSphere {
    pub center: Vec3,
    pub radius: f32,
}

impl ColliderArea for ColliderSphere {
    fn in_frustum(&self, transform: &Transform, frustum: &[Vec4; 6]) -> bool {
        let distances: Vec<_> = frustum
            .iter()
            .map(|plane| plane.dot(self.center.extend(1.0)))
            .collect();

        if distances.iter().any(|&d| d < -self.radius) {
            return false; // Sphere is completely outside the frustum
        }
        if distances.iter().all(|&d| d < self.radius) {
            return true; // Sphere is completely inside the frustum
        }

        // Otherwise, the sphere intersects the frustum
        true
    }
}

struct ColliderRectBox {
    pub center: Vec3,
    pub half_width: f32,
    pub half_height: f32,
    pub half_depth: f32,
}

impl ColliderArea for ColliderRectBox {
    fn in_frustum(&self, transform: &Transform,frustum: &[Vec4; 6]) -> bool {
        let half_diagonal_vec = Vec4::new(self.half_width, self.half_height, self.half_depth, 1.0);
        let half_diagonal = half_diagonal_vec.length();
        let distances: Vec<_> = frustum
            .iter()
            .map(|plane| plane.dot(self.center.extend(1.0) + half_diagonal_vec))
            .collect();
        if distances.iter().any(|&d| d < -half_diagonal) {
            return false; // Box is completely outside the frustum
        }
        if distances.iter().all(|&d| d < half_diagonal) {
            return true; // Box is completely inside the frustum
        }

        // Otherwise, the box intersects the frustum
        true
    }
}

struct ColliderCylinder {
    pub center: Vec3,
    pub radius: f32,
    pub half_height: f32,
}

impl ColliderArea for ColliderCylinder {
    fn in_frustum(&self, transform: &Transform, frustum: &[Vec4; 6]) -> bool {
        // Calculate the top and bottom center points of the cylinder
        let top_center = self.center + Vec3::new(0.0, self.half_height, 0.0);
        let bottom_center = self.center - Vec3::new(0.0, self.half_height, 0.0);

        // Check each plane of the frustum
        for plane in frustum {
            let center_distance = plane.dot(self.center.extend(1.0));
            let top_distance = plane.dot(top_center.extend(1.0));
            let bottom_distance = plane.dot(bottom_center.extend(1.0));

            if center_distance < -self.radius && top_distance < 0.0 && bottom_distance < 0.0 {
                return false; // Cylinder is completely outside the frustum
            }
        }

        true // Cylinder intersects or is completely inside the frustum
    }
}

struct BoundingCollider {
    pub vertices: Vec<Vec3>,
}

impl ColliderArea for BoundingCollider {
    fn in_frustum(&self, transform: &Transform, frustum: &[Vec4; 6]) -> bool {
        for plane in frustum {
            let mut all_outside = true;
            for vertex in &self.vertices {
                let distance = plane.dot(vertex.extend(1.0));
                if distance >= 0.0 {
                    all_outside = false;
                    break;
                }
            }
            if all_outside {
                return false; // All vertices are outside the plane
            }
        }
        true // All vertices are inside the frustum
    }
}
