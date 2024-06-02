use glam::Mat4;
use glfw::Key;

use crate::engine::scene::gameobject::components::{Component, ComponentMap};
use crate::engine::scene::gameobject::GameObjectData;
use crate::engine::GameState;
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

pub struct FreeCamController {}

impl FreeCamController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for FreeCamController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn step(
        &mut self,
        object: &mut GameObjectData,
        components: &ComponentMap,
        state: &GameState,
    ) -> EngineStepResult<()> {
        // let poses = vr_system
        //     .device_to_absolute_tracking_pose(openvr::TrackingUniverseOrigin::RawAndUncalibrated, 0.0);
        // let head_pose = poses[0];
        // let transform = Transform::from(to_mat4_3x4(&head_pose.device_to_absolute_tracking()).inverse());
        // self.data_mut().transform.rotation = transform.rotation;
        let speed = 10.0 * state.delta.as_secs_f32();
        let rotation_speed = 0.1 * state.delta.as_secs_f32();
        let forward = object.transform.forward();
        let right = object.transform.right();
        let transform = &mut object.transform;
        if state.input_state.keyboard.is_held(Key::W) {
            transform.position += forward * speed;
        }
        if state.input_state.keyboard.is_held(Key::S) {
            transform.position -= forward * speed;
        }
        if state.input_state.keyboard.is_held(Key::D) {
            transform.position += right * speed;
        }
        if state.input_state.keyboard.is_held(Key::A) {
            transform.position -= right * speed;
        }
        if state.input_state.keyboard.is_held(Key::Space) {
            transform.position.y += speed;
        }
        if state.input_state.keyboard.is_held(Key::LeftShift) {
            transform.position.y -= speed;
        }
        transform.rotation *=
            glam::Quat::from_rotation_x(rotation_speed * state.input_state.mouse_delta.1 as f32);
        transform.rotation =
            glam::Quat::from_rotation_y(rotation_speed * -state.input_state.mouse_delta.0 as f32)
                * transform.rotation;
        if state.input_state.keyboard.is_held(Key::Q) {
            transform.rotation *= glam::Quat::from_rotation_z(speed * 0.1);
        }
        if state.input_state.keyboard.is_held(Key::E) {
            transform.rotation *= glam::Quat::from_rotation_z(-speed * 0.1);
        }
        Ok(())
    }
}
