use glengine::engine::scene::camera::CameraControlled;
use glengine::engine::scene::gameobject::RotatingGameObject;
use glengine::engine::scene::gameobject::BaseGameObject;
use glengine::engine::components::{CompA,CompB};
use glengine::engine::scene::lights::Lights;
use std::sync::{Arc, Mutex,Weak};
use glengine::engine::drawable::base::Drawable;
use glengine::engine::drawable::importer::obj;
use glengine::engine::Engine;
use glengine::engine::GameData;
use glengine::engine::scene::Scene;
use glengine::engine::scene::SceneData;
use glengine::gl;
use glengine::glam::vec3;
use glengine::result::EngineRenderResult;


struct BaseScene {
    data: SceneData,
}

impl BaseScene {
    fn new() -> Self {
        Self {
            data: SceneData {
                objects: Vec::new(),
                main_camera: Weak::new(),
                lights: Lights::default(),
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

    fn init_gl(&mut self) -> EngineRenderResult<()> {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }
        let empty = BaseGameObject::new(None);
        self.data.objects.push(empty.clone());

        let monkey = BaseGameObject::new(Some(empty.clone()));
        {
            let monkey_draw = obj::import("C:\\Users\\alx\\RustroverProjects\\rustgraphics\\monkeyhp.obj");
            let mut data = monkey.lock().expect("Could not lock gameobject for init");
            data.data_mut().drawable = Some(Box::new(monkey_draw));
            data.data_mut().transform.position = vec3(5.0, 2.0, -4.0);
            data.data_mut().transform.scale *= 5.0;
        }

        let floor = BaseGameObject::new(Some(empty.clone()));
        {
            let mut data = floor.lock().expect("Could not lock gameobject for init");
            data.data_mut().drawable = Some(Box::new(Drawable::default()));
            data.data_mut().transform.scale.x *= 200.0;
            data.data_mut().transform.scale.z *= 200.0;
            data.data_mut().transform.position = vec3(0.0, -8.0, 0.0);
        }

        let rotator = RotatingGameObject::new(Some(empty.clone()), vec3(0.0, 0.0, 0.0));
        {
            let mut data = rotator.lock().expect("Could not lock gameobject for init");
            let drawable = obj::import(
                "C:\\Users\\alx\\RustroverProjects\\rustgraphics\\bugatticlean.obj",
            );
            data.data_mut().drawable = Some(Box::new(drawable));
            data.data_mut().transform.scale *= 0.3;
            data.data_mut().transform.position = vec3(0.0, 0.0, 0.0);
        }

        let camera = Arc::new(Mutex::new(CameraControlled::new(
            None,
            vec3(20.0, 20.0, 20.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        )));
        self.data.objects.push(camera.clone());
        self.data.main_camera = Arc::downgrade(&camera);
        Ok(())
    }
}

fn main() {
    let mut engine = Engine::from_game(GameData::new(Some(Box::new(BaseScene::new()))));
    if let Err(e) = engine.run() {
        eprintln!("Error: {:?}", e);
    }
}
