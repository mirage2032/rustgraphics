use glam::{Vec3, vec3};

pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

// impl From<TMat4<f32>> for Transform{
//     fn from(mat: TMat4<f32>) -> Self {
//
//
//         Self {
//             position,
//             rotation,
//             scale,
//         }
//     }
// }

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
            scale: vec3(1.0, 1.0, 1.0),
        }
    }
}