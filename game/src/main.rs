use glengine::engine::scene::gameobject::components::rotating::RotatingComponent;
use std::rc::{Rc};

use glengine::engine::drawable::base::BaseDrawable;
use glengine::engine::drawable::material::{Material, MaterialData};
use glengine::engine::drawable::shader::lit::LIT_COLOR_SHADER;
use glengine::engine::Engine;
use glengine::engine::GameData;
use glengine::engine::scene::camera::Camera;
use glengine::engine::scene::gameobject::{base::BaseGameObject};
use glengine::engine::scene::gameobject::components::drawable::DrawableComponent;
use glengine::engine::scene::lights::directional::DirectionalLight;
use glengine::engine::scene::lights::Lights;
use glengine::engine::scene::lights::point::PointLight;
use glengine::engine::scene::lights::spot::SpotLight;
use glengine::engine::scene::Scene;
use glengine::engine::scene::SceneData;
use glengine::{gl, nmdl_import};
use glengine::engine::transform::Transform;
use glengine::glam::{Mat4, vec3, Quat};
use glengine::result::EngineRenderResult;

struct BaseScene {
    data: SceneData,
}

impl BaseScene {
    fn new() -> Self {
        Self {
            data: SceneData {
                objects: Vec::new(),
                main_camera: None,
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
        let models_dir = std::env::current_exe().unwrap().parent().unwrap().join("models");
        self.data_mut().lights.init_ssbo();
        unsafe { 
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        }
        let empty = BaseGameObject::new(None);
        self.data.objects.push(empty.clone());
        // nmdl_import!("Bkaf.obj");  
        let monkey = BaseGameObject::new(Some(empty.clone()));
        {
            let monkey_draw = nmdl_import!("untitled.obj");
            let mut data = monkey.borrow_mut();
            data.components
                .add_component(DrawableComponent::new(Box::new(monkey_draw)));
            data.data.transform.position = vec3(5.0, 2.0, -4.0);
            data.data.transform.scale *= 5.0;
        }

        let floor = BaseGameObject::new(Some(empty.clone()));
        {
            let mut drawable = BaseDrawable::default();
            drawable.draw_data[0].shader = LIT_COLOR_SHADER.clone();
            drawable.draw_data[0].material = Some(Rc::new(Material {
                data: MaterialData {
                    ambient: Some(vec3(0.3, 0.1, 0.1)),
                    diffuse: Some(vec3(1.0, 0.4, 0.6)),
                    specular: Some(vec3(1.0, 0.5, 0.7)),
                    shininess: Some(0.02),
                },
                textures: Default::default(),
            }));
            let mut data = floor.borrow_mut();
            data.components
                .add_component(DrawableComponent::new(Box::new(drawable)));
            data.data.transform.scale *= 200.0;
            data.data.transform.scale.y *= 0.001;
            data.data.transform.position = vec3(0.0, -4.0, 0.0);
        }

        let cube = BaseGameObject::new(Some(empty.clone()));
        {
            let mut drawable = BaseDrawable::default();
            drawable.draw_data[0].shader = LIT_COLOR_SHADER.clone();
            drawable.draw_data[0].material = Some(Rc::new(Material {
                data: MaterialData {
                    ambient: Some(vec3(0.9, 0.1, 0.1)),
                    diffuse: Some(vec3(1.0, 0.4, 0.6)),
                    specular: Some(vec3(1.0, 0.5, 0.7)),
                    shininess: Some(0.02),
                },
                textures: Default::default(),
            }));
            let mut data = cube.borrow_mut();
            data.components
                .add_component(DrawableComponent::new(Box::new(drawable)));
            data.data.transform.scale *= 4.0;
            data.data.transform.position = vec3(0.0, -1.0, 8.0);
        }

        let rotator = BaseGameObject::new(Some(empty.clone()));
        {
            let mut data = rotator.borrow_mut();
            let drawable = nmdl_import!("bugatticlean.obj");
            data.components.add_component(DrawableComponent::new(Box::new(drawable)));
            data.components.add_component(RotatingComponent::new(vec3(0.0, 0.14, 0.0)));
            data.data.transform.scale *= 0.3;
            data.data.transform.position = vec3(0.0, 0.0, 0.0);
        }

        let camera = Camera::new(
            None,
            vec3(20.0, 20.0, 20.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );

        let spot_light = SpotLight::new(
            Some(camera.game_object.clone()),
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
            .push(spot_light);
        
        let mut point_light_gameobject = BaseGameObject::new_w_transform(Some(empty.clone()),
        Transform::default().with_position(vec3(5.0, 13.0, -20.0)));

        let point_light = PointLight::new_w_gameobject(
            point_light_gameobject.clone(),
            1.8,
            vec3(1.0, 1.0, 1.0),
            1.0,
            0.09,
            0.032,
        );
        self.data_mut()
            .lights
            .point
            .push(point_light);
        
        let direction_light_gameobject = BaseGameObject::new_w_transform(Some(empty.clone()), Mat4::from_quat(Quat::from_axis_angle(vec3(1.0, 0.6, 0.8), -std::f32::consts::FRAC_PI_2)).into());
        let directional_light =
            DirectionalLight::new_w_gameobject(direction_light_gameobject, 0.04, vec3(1.0, 1.0, 1.0));

        self.data_mut()
            .lights
            .directional = Some(directional_light);

        self.data.objects.push(camera.game_object.clone());
        self.data.main_camera = Some(camera);
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
