use std::sync::{Arc, RwLock, Weak};

use glengine::engine::drawable::base::BaseDrawable;
use glengine::engine::drawable::importer::assimp;
use glengine::engine::drawable::material::{Material, MaterialData};
use glengine::engine::drawable::shader::lit::LIT_COLOR_SHADER;
use glengine::engine::Engine;
use glengine::engine::GameData;
use glengine::engine::scene::camera::CameraControlled;
use glengine::engine::scene::gameobject::{base::BaseGameObject, GameObjectTrait};
use glengine::engine::scene::gameobject::components::drawable::DrawableComponent;
use glengine::engine::scene::gameobject::components::rotating::RotatingComponent;
use glengine::engine::scene::lights::directional::DirectionalLight;
use glengine::engine::scene::lights::Lights;
use glengine::engine::scene::lights::point::PointLight;
use glengine::engine::scene::lights::spot::SpotLight;
use glengine::engine::scene::Scene;
use glengine::engine::scene::SceneData;
use glengine::gl;
use glengine::glam::{Mat4, vec3,Quat};
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
        self.data_mut().lights.init_ssbo();
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }
        let empty = BaseGameObject::new(None);
        self.data.objects.push(empty.clone());

        let monkey = BaseGameObject::new(Some(empty.clone()));
        {
            let monkey_draw =
                assimp::import("models/untitled.obj");
            let mut data = monkey.write().expect("Could not lock gameobject for init");
            data.components_mut()
                .unwrap()
                .add_component(DrawableComponent::new(Box::new(monkey_draw)));
            data.data_mut().transform.position = vec3(5.0, 2.0, -4.0);
            data.data_mut().transform.scale *= 5.0;
        }

        let floor = BaseGameObject::new(Some(empty.clone()));
        {
            let mut drawable = BaseDrawable::default();
            drawable.draw_data[0].shader = LIT_COLOR_SHADER.clone();
            drawable.draw_data[0].material = Some(Arc::new(Material {
                data: MaterialData {
                    ambient: Some(vec3(0.3, 0.1, 0.1)),
                    diffuse: Some(vec3(1.0, 0.4, 0.6)),
                    specular: Some(vec3(1.0, 0.5, 0.7)),
                    shininess: Some(0.02),
                },
                textures: Default::default(),
            }));
            let mut data = floor.write().expect("Could not lock gameobject for init");
            data.components_mut()
                .unwrap()
                .add_component(DrawableComponent::new(Box::new(drawable)));
            data.data_mut().transform.scale *= 200.0;
            data.data_mut().transform.scale.y *= 0.001;
            data.data_mut().transform.position = vec3(0.0, -4.0, 0.0);
        }

        let cube = BaseGameObject::new(Some(empty.clone()));
        {
            let mut drawable = BaseDrawable::default();
            drawable.draw_data[0].shader = LIT_COLOR_SHADER.clone();
            drawable.draw_data[0].material = Some(Arc::new(Material {
                data: MaterialData {
                    ambient: Some(vec3(0.9, 0.1, 0.1)),
                    diffuse: Some(vec3(1.0, 0.4, 0.6)),
                    specular: Some(vec3(1.0, 0.5, 0.7)),
                    shininess: Some(0.02),
                },
                textures: Default::default(),
            }));
            let mut data = cube.write().expect("Could not lock gameobject for init");
            data.components_mut()
                .unwrap()
                .add_component(DrawableComponent::new(Box::new(drawable)));
            data.data_mut().transform.scale *= 4.0;
            data.data_mut().transform.position = vec3(0.0, -1.0, 8.0);
        }

        // let rotator = BaseGameObject::new(Some(empty.clone()));
        // {
        //     let mut data = rotator.write().expect("Could not lock gameobject for init");
        //     let components = data.components_mut().unwrap();
        //     let drawable =
        //         assimp::import("models/bugatticlean.obj");
        //     components.add_component(DrawableComponent::new(Box::new(drawable)));
        //     components.add_component(RotatingComponent::new(vec3(0.0, 0.04, 0.0)));
        //     data.data_mut().transform.scale *= 0.3;
        //     data.data_mut().transform.position = vec3(0.0, 0.0, 0.0);
        // }

        let camera = Arc::new(RwLock::new(CameraControlled::new(
            None,
            vec3(20.0, 20.0, 20.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        )));

        let spot_light = SpotLight::new(
            Some(camera.clone()),
            1.0,
            vec3(1.0, 1.0, 1.0),
            1.0,
            0.09,
            0.032,
            27.0,
            50.0,
        );
        self.data_mut()
            .lights
            .spot
            .push(Arc::downgrade(&spot_light));

        let point_light = PointLight::new(
            Some(empty.clone()),
            1.8,
            vec3(1.0, 1.0, 1.0),
            1.0,
            0.09,
            0.032,
        );
        point_light.write().unwrap().data_mut().transform.position = vec3(5.0, 13.0, -20.0);
        self.data_mut()
            .lights
            .point
            .push(Arc::downgrade(&point_light));

        let directional_light =
            DirectionalLight::new(Some(empty.clone()), 0.04, vec3(1.0, 1.0, 1.0));
        {
            //mat4 pointing down
            let mat = Mat4::from_quat(Quat::from_axis_angle(vec3(1.0, 0.6, 0.8), -std::f32::consts::FRAC_PI_2));
            directional_light
                .write()
                .unwrap()
                .data_mut()
                .transform = mat.into();
        }

        self.data_mut()
            .lights
            .directional
            = Arc::downgrade(&directional_light);

        self.data.objects.push(camera.clone());
        self.data.main_camera = Arc::downgrade(&camera);
        Ok(())
    }
}

fn main() {
    let mut engine = Engine::from_game(GameData::new(Some(Box::new(BaseScene::new()))));
    if let Err(e) = engine.run() {
        eprintln!("Error: {:?}", e);
        let s = unsafe { gl::GetError() };
        eprintln!("Error: {:?}", s);
    }
}
