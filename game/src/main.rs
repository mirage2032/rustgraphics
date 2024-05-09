mod drawable;

use glengine::engine::shader::Shader;
use crate::drawable::shader::arrayshader::build_array_shader;
use drawable::mesh::arraymesh;

use rand::Rng;
use std::sync::{Arc, RwLock};
use glengine::gl;


use glengine::engine::camera::CameraControlled;
use glengine::engine::drawable::base::BaseDrawable;
use glengine::engine::drawable::mesh::cube::CubeMesh;
use glengine::engine::drawable::mesh::Mesh;
use glengine::engine::drawable::mesh::model::ModelMesh;
use glengine::engine::Engine;
use glengine::engine::GameData;
use glengine::engine::gameobject::{RotatingGameObject, GameObjectRaw,BaseGameObject};
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

        //let monkey_mesh: Arc<Box<dyn Mesh>> = Arc::new(Box::new(ModelMesh::new("C:\\Users\\alx\\RustroverProjects\\rustgraphics\\untitled.obj")));
        let array_monkey_mesh: Arc<Box<dyn Mesh>> = Arc::new(Box::new(arraymesh::ArrayMesh::new(Box::new(ModelMesh::new("C:\\Users\\alx\\RustroverProjects\\rustgraphics\\untitled.obj")), 50, 20, 20)));
        let cube_mesh: Arc<Box<dyn Mesh>> = Arc::new(Box::new(CubeMesh::default()));
        let def_shader = Arc::new(glengine::engine::shader::Shader::default());
        let empty = BaseGameObject::new(None);
        self.data.objects.push(empty.clone());

        let monkey = BaseGameObject::new(Some(empty.clone()));
        {
            let mut data = monkey.write().expect("Could not lock gameobject for init");
            data.data_mut().drawable = Some(Box::new(BaseDrawable::new(array_monkey_mesh.clone(), Arc::new(build_array_shader()))));
            // data.data_mut().transform.rotation *= Quat::from_rotation_x(-20.0_f32.to_radians());
            // data.data_mut().transform.rotation *= Quat::from_rotation_y(-35.0_f32.to_radians());
            data.data_mut().transform.scale *= 0.1;
        }

        let cubea = BaseGameObject::new(Some(empty.clone()));
        {
            let mut data = cubea.write().expect("Could not lock gameobject for init");
            data.data_mut().drawable = Some(Box::new(BaseDrawable::default()));
            data.data_mut().transform.scale.x *= 200.0;
            data.data_mut().transform.scale.z *= 200.0;
            data.data_mut().transform.position = vec3(0.0, -8.0, 0.0);
        }

        let obj2 = RotatingGameObject::new(Some(empty.clone()), vec3(0.0, 0.0, 2.0));
        {
            let mut data = obj2.write().expect("Could not lock gameobject for init");
            let mesh:Arc<Box<dyn Mesh>> = Arc::new(Box::new(ModelMesh::new("C:\\Users\\alx\\RustroverProjects\\rustgraphics\\untitled.obj")));
            data.data_mut().drawable = Some(Box::new(BaseDrawable::new(Arc::new(Box::new(ModelMesh::new("C:\\Users\\alx\\RustroverProjects\\rustgraphics\\untitled.obj"))), def_shader.clone())));
            data.data_mut().transform.scale *= 9.08;
            data.data_mut().transform.position = vec3(15.0, 0.0, 0.0);
            data.data_mut().transform.rotation *= Quat::from_rotation_x(-60.0_f32.to_radians());
            data.data_mut().transform.rotation *= Quat::from_rotation_y(-60.0_f32.to_radians());
        }

        let camera = Arc::new(RwLock::new(CameraControlled::new(
            None,
            vec3(0.0, -4.0, 20.0),
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