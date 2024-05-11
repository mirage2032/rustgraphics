use std::sync::{Arc, Mutex};

use drawable::mesh::arraymesh;
use glengine::engine::camera::CameraControlled;
use glengine::engine::drawable::base::BaseDrawable;
use glengine::engine::drawable::mesh::cube::CubeMesh;
use glengine::engine::drawable::mesh::Mesh;
use glengine::engine::drawable::mesh::model::ModelMesh;
use glengine::engine::Engine;
use glengine::engine::GameData;
use glengine::engine::gameobject::{BaseGameObject, RotatingGameObject};
use glengine::engine::scene::Scene;
use glengine::engine::scene::SceneData;
use glengine::gl;
use glengine::glam::vec3;

use crate::drawable::shader::arrayshader::build_array_shader;

mod drawable;

struct BaseScene {
    data: SceneData,
}

impl BaseScene {
    fn new() -> Self {
        Self {
            data: SceneData {
                objects: Vec::new(),
                main_camera: None,
            },
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

        //let monkey_mesh: Arc<Box<dyn Mesh>> = Arc::new(Box::new(ModelMesh::new("C:\\Users\\alx\\RustroverProjects\\rustgraphics\\untitled.obj")));
        let array_monkey_mesh: Arc<Box<dyn Mesh>> = Arc::new(Box::new(arraymesh::ArrayMesh::new(
            Box::new(ModelMesh::new(
                "C:\\Users\\alx\\RustroverProjects\\rustgraphics\\untitled.obj",
            )),
            50,
            20,
            20,
        )));
        let cube_mesh: Arc<Box<dyn Mesh>> = Arc::new(Box::new(CubeMesh::default()));
        let def_shader = Arc::new(glengine::engine::shader::Shader::default());
        let empty = BaseGameObject::new(None);
        self.data.objects.push(empty.clone());

        let array_monkey = BaseGameObject::new(Some(empty.clone()));
        {
            let mut data = array_monkey
                .lock()
                .expect("Could not lock gameobject for init");
            data.data_mut().drawable = Some(Box::new(BaseDrawable::new(
                array_monkey_mesh.clone(),
                Arc::new(build_array_shader()),
            )));
            data.data_mut().transform.position = vec3(10.0, 0.0, -20.0);
            data.data_mut().transform.scale *= 0.1;
        }

        let floor = BaseGameObject::new(Some(empty.clone()));
        {
            let mut data = floor.lock().expect("Could not lock gameobject for init");
            data.data_mut().drawable = Some(Box::new(BaseDrawable::default()));
            data.data_mut().transform.scale.x *= 200.0;
            data.data_mut().transform.scale.z *= 200.0;
            data.data_mut().transform.position = vec3(0.0, -8.0, 0.0);
        }

        let rotator = RotatingGameObject::new(Some(empty.clone()), vec3(0.3, 0.5, 0.6));
        {
            let mut data = rotator.lock().expect("Could not lock gameobject for init");
            let mesh: Arc<Box<dyn Mesh>> = Arc::new(Box::new(ModelMesh::new(
                "C:\\Users\\alx\\RustroverProjects\\rustgraphics\\untitled.obj",
            )));
            data.data_mut().drawable = Some(Box::new(BaseDrawable::new(mesh, def_shader.clone())));
            data.data_mut().transform.scale *= 4.2;
            data.data_mut().transform.position = vec3(0.0, 0.0, 0.0);
        }

        let camera = Arc::new(Mutex::new(CameraControlled::new(
            None,
            vec3(20.0, 20.0, 20.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        )));

        self.data.objects.push(camera.clone());
        self.data.main_camera = Some(camera);
    }
}

fn main() {
    let mut engine = Engine::from_game(GameData::new(Some(Box::new(BaseScene::new()))));
    engine.run();
}
