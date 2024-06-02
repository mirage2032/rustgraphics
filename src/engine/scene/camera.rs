use std::sync::Mutex;
use glam::{Mat4, Vec3};
use glfw::Key;
use lazy_static::lazy_static;
extern crate openvr;
use crate::engine::{GameState};
use crate::engine::scene::gameobject::{GameObject, GameObjectData, GameObjectRaw};
use crate::engine::transform::Transform;
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
}

impl CameraControlled {
    pub fn new(parent: Option<GameObject>, position: Vec3, target: Vec3, up: Vec3) -> Self {
        let mut data = GameObjectData::new(parent);
        data.transform = Mat4::look_at_rh(position, target, up).inverse().into();
        Self { data: data }
    }
}

impl Default for CameraControlled {
    fn default() -> Self {
        Self {
            data: GameObjectData::new(None),
        }
    }
}

impl GameObjectRaw for CameraControlled {
    fn data(&self) -> &GameObjectData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut GameObjectData {
        &mut self.data
    }

    fn step(&mut self, game: &GameState) -> EngineStepResult<()> {
        // let poses = vr_system
        //     .device_to_absolute_tracking_pose(openvr::TrackingUniverseOrigin::RawAndUncalibrated, 0.0);
        // let head_pose = poses[0];
        // let transform = Transform::from(to_mat4_3x4(&head_pose.device_to_absolute_tracking()).inverse());
        // self.data_mut().transform.rotation = transform.rotation;
        
        
        let speed = 10.0 * game.delta.as_secs_f32();
        let rotation_speed = 0.1 * game.delta.as_secs_f32();
        let forward = self.data.transform.forward();
        let right = self.data.transform.right();
        let transform = &mut self.data.transform;
        if game.input_state.keyboard.is_held(Key::W) {
            transform.position += forward * speed;
        }
        if game.input_state.keyboard.is_held(Key::S) {
            transform.position -= forward * speed;
        }
        if game.input_state.keyboard.is_held(Key::D) {
            transform.position += right * speed;
        }
        if game.input_state.keyboard.is_held(Key::A) {
            transform.position -= right * speed;
        }
        if game.input_state.keyboard.is_held(Key::Space) {
            transform.position.y += speed;
        }
        if game.input_state.keyboard.is_held(Key::LeftShift) {
            transform.position.y -= speed;
        }
        transform.rotation *=
            glam::Quat::from_rotation_x(rotation_speed * game.input_state.mouse_delta.1 as f32);
        transform.rotation =
            glam::Quat::from_rotation_y(rotation_speed * -game.input_state.mouse_delta.0 as f32) * transform.rotation;
        if game.input_state.keyboard.is_held(Key::Q) {
            transform.rotation *= glam::Quat::from_rotation_z(speed * 0.1);
        }
        if game.input_state.keyboard.is_held(Key::E) {
            transform.rotation *= glam::Quat::from_rotation_z(-speed * 0.1);
        }
        Ok(())
    }
}
