use rand::Rng;
use std::sync::{Arc, RwLock};
use glengine::gl;

use glengine::engine::camera::PointCamera;
use glengine::engine::drawable::base::BaseDrawable;
use glengine::engine::drawable::mesh::Mesh;
use glengine::engine::drawable::mesh::model::ModelMesh;
use glengine::engine::Engine;
use glengine::engine::GameData;
use glengine::engine::gameobject::{BaseGameObject, GameObjectRaw};
use glengine::engine::scene::Scene;
use glengine::engine::scene::SceneData;
use glengine::glam::{vec3,Quat};

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
        let mesh:Box<dyn Mesh> = Box::new(glengine::engine::drawable::mesh::cube::CubeMesh::default());
        let mesh = Arc::new(mesh);
        let shader = Arc::new(glengine::engine::shader::Shader::default());
        let mut empty = BaseGameObject::new(None, vec3(0.0, 0.0, 0.3));
        let length = 100;
        let scale = 0.05;
        let offset = 0.10;
        for offset_y in -length..length {
            for offset_x in -length..length {
                let rot_x = rng.gen_range(0.0001..1.7);
                let rot_y = rng.gen_range(0.0001..1.5);
                let rot_z = rng.gen_range(0.0001..1.3);
                let mut cubeobj = BaseGameObject::new(None, vec3(rot_x, rot_y, rot_z));
                let data = cubeobj.data_mut();
                data.transform.scale *= scale;
                data.transform.position += vec3(offset * offset_x as f32, offset * offset_y as f32, 0.0);
                data.drawable = Some(Box::new(BaseDrawable::new(mesh.clone(),shader.clone())));
                empty.data_mut().children.push(Arc::new(RwLock::new(cubeobj)));
            }
        }
        self.data.objects.push(Arc::new(RwLock::new(empty)));

        let mut monkey = BaseGameObject::new(None, vec3(1.6, 1.2, 3.2));
        let data = monkey.data_mut();
        data.drawable = Some(Box::new(BaseDrawable::new(Arc::new(Box::new(ModelMesh::new("C:\\Users\\alx\\RustroverProjects\\rustgraphics\\untitled.obj"))), Arc::new(glengine::engine::shader::Shader::default()))));
        data.transform.rotation *= Quat::from_rotation_x(-20.0_f32.to_radians());
        data.transform.rotation *= Quat::from_rotation_y(-35.0_f32.to_radians());
        data.transform.scale = vec3(1.0, 1.0, 1.0);
        self.data.objects.push(Arc::new(RwLock::new(monkey)));

        let camera = Arc::new(RwLock::new(PointCamera::new(
            None,
            vec3(2.0, 2.0, 3.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        )));

        self.data.objects.push(camera.clone());
        self.data.main_camera = Some(camera);

    }
}

fn main() {
    let mut engine = Engine::from_game(GameData { scene: Some(Box::new(BaseScene::new())) });
    engine.run();
}