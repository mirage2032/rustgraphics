use glam::{Mat4, Quat, Vec3};

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

    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    pub fn forward(&self) -> Vec3 {
        let xyzrotation = self.rotation.to_axis_angle();
        let new =self.rotation * Vec3::Z;
        let xyzrotation = self.rotation.to_axis_angle();
        new
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * -Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
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

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}
