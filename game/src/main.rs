use rand::Rng;
use std::sync::Arc;
use glengine::gl;

use glengine::engine::camera::Camera;
use glengine::engine::drawable::base::BaseDrawable;
use glengine::engine::drawable::mesh::MeshTrait;
use glengine::engine::Engine;
use glengine::engine::GameData;
use glengine::engine::gameobject::{BaseGameObject, GameObject};
use glengine::engine::scene::Scene;
use glengine::engine::scene::SceneData;
use glengine::engine::transform::Transform;
use glengine::glam::{Mat4, vec3};

struct BaseScene {
    data: SceneData,
}

impl BaseScene {
    fn new() -> Self {
        Self {
            data: SceneData {
                objects: Vec::new(),
                main_camera: None,
            }
        }
    }
}

impl Scene for BaseScene {
    fn data(&self) -> &SceneData {
        &self.data
    }

    fn data_mut(&mut self) -> &mut SceneData {
        &mut self.data
    }

    fn init_gl(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }

        let mut rng = rand::thread_rng();
        let mesh:Box<dyn MeshTrait> = Box::new(glengine::engine::drawable::mesh::cube::CubeMesh::default());
        let mesh = Arc::new(mesh);
        let shader = Arc::new(glengine::engine::shader::Shader::default());
        for offset_y in -20..20 {
            for offset_x in -20..20 {
                let rot_x = rng.gen_range(0.0001..1.7);
                let rot_y = rng.gen_range(0.0001..1.5);
                let rot_z = rng.gen_range(0.0001..1.3);
                let mut cubeobj: Box<dyn GameObject> = Box::new(BaseGameObject::new(None, vec3(rot_x, rot_y, rot_z)));
                let data = cubeobj.data_mut();
                data.transform.scale = vec3(0.1, 0.1, 0.1);
                data.transform.position += vec3(0.20 * offset_x as f32, 0.20 * offset_y as f32, 0.0);
                data.drawable = Some(Box::new(BaseDrawable::new(mesh.clone(),shader.clone())));
                self.data.objects.push(cubeobj);
            }
        }

        let rotated_view = {
            let eye = vec3(0.0, 0.0, 3.0); // Camera position
            let center = vec3(0.0, 0.0, 0.0); // Camera target
            let up = vec3(0.0, 1.0, 0.0); // Up vector

            // Create a view matrix
            let view = Mat4::look_at_rh(eye, center, up);

            // Define rotation parameters
            let angle = 15.0_f32.to_radians(); // Rotation angle in degrees
            let axis = vec3(0.0, 1.0, 0.0); // Rotation axis

            // Create a rotation matrix
            let rotation = Mat4::from_axis_angle(axis, angle);
            view * rotation
        };
        self.data.main_camera = Some(Camera::new(Transform::from(rotated_view)));
    }
}

fn main() {
    let mut engine = Engine::from_game(GameData { scene: Some(Box::new(BaseScene::new())) });
    engine.run();
}