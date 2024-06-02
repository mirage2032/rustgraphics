extern crate openvr;

use glam::{Mat4, Vec3, Vec4};

use crate::engine::config::CONFIG;
use crate::engine::GameState;
use crate::engine::scene::gameobject::{GameObject, GameObjectData, GameObjectTrait};
use crate::engine::scene::gameobject::components::{ComponentMap, freecam};
use crate::result::EngineStepResult;

fn to_mat4(mat: [[f32; 4]; 4]) -> Mat4 {
    Mat4::from_cols_array_2d(&mat)
}

fn to_mat4_3x4(mat: &[[f32; 4]; 3]) -> Mat4 {
    Mat4::from_cols_array_2d(&[
        [mat[0][0], mat[0][1], mat[0][2], 0.0],
        [mat[1][0], mat[1][1], mat[1][2], 0.0],
        [mat[2][0], mat[2][1], mat[2][2], 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}
pub struct CameraControlled {
    pub data: GameObjectData,
    pub components: ComponentMap,
}

impl CameraControlled {
    pub fn new(parent: Option<GameObject>, position: Vec3, target: Vec3, up: Vec3) -> Self {
        let mut data = GameObjectData::new(parent);
        data.transform = Mat4::look_at_rh(position, target, up).inverse().into();
        let mut components = ComponentMap::new();
        components.add_component(freecam::FreeCamController {});
        Self {
            data,
            components,
        }
    }

    pub fn frustum(&self) -> Mat4 {
        let perspective = *CONFIG.read().expect("Failed to read config").projection();
        perspective * self.global_mat().inverse()
    }

    pub fn frustum_planes(&self) -> [Vec4; 6] {
        let frustum = self.frustum();
        let left_plane = frustum.row(3) + frustum.row(0);
        let right_plane = frustum.row(3) - frustum.row(0);
        let bottom_plane = frustum.row(3) + frustum.row(1);
        let top_plane = frustum.row(3) - frustum.row(1);
        let near_plane = frustum.row(3) + frustum.row(2);
        let far_plane = frustum.row(3) - frustum.row(2);
        [
            left_plane,
            right_plane,
            bottom_plane,
            top_plane,
            near_plane,
            far_plane,
        ]
    }
}

impl Default for CameraControlled {
    fn default() -> Self {
        Self {
            data: GameObjectData::new(None),
            components: ComponentMap::new(),
        }
    }
}

impl GameObjectTrait for CameraControlled {
    fn data(&self) -> &GameObjectData {
        &self.data
    }
    fn data_mut(&mut self) -> &mut GameObjectData {
        &mut self.data
    }

    fn components(&self) -> Option<&ComponentMap> {
        Some(&self.components)
    }

    fn components_mut(&mut self) -> Option<&mut ComponentMap> {
        Some(&mut self.components)
    }

    fn step(&mut self, state: &GameState) -> EngineStepResult<()> {
        self.components.step(&mut self.data, state)?;
        Ok(())
    }
}
