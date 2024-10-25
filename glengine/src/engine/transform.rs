use std::hash::{Hash, Hasher};
use glam::{Mat4, Quat, Vec3};

#[derive(Debug, Clone, Copy,PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * -Vec3::Z
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }
    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }
    pub fn with_rotation(mut self, rotation:Quat) -> Self {
        self.rotation = rotation;
        self
    }
    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }
}

impl From<Transform> for Mat4 {
    fn from(transform: Transform) -> Self {
        Mat4::from_scale_rotation_translation(
            transform.scale,
            transform.rotation,
            transform.position,
        )
    }
}

impl From<Mat4> for Transform {
    fn from(mat: Mat4) -> Self {
        let (scale, rotation, position) = mat.to_scale_rotation_translation();
        Self {
            position,
            rotation,
            scale,
        }
    }
}

impl Hash for Transform {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.position.x.to_bits());
        state.write_u32(self.position.y.to_bits());
        state.write_u32(self.position.z.to_bits());

        state.write_u32(self.rotation.x.to_bits());
        state.write_u32(self.rotation.y.to_bits());
        state.write_u32(self.rotation.z.to_bits());
        state.write_u32(self.rotation.w.to_bits());

        state.write_u32(self.scale.x.to_bits());
        state.write_u32(self.scale.y.to_bits());
        state.write_u32(self.scale.z.to_bits());
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}
