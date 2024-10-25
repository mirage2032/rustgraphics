use glam::Vec3;
use glengine::engine::drawable::base::BaseDrawable;
use glengine::engine::drawable::importer::nmdl::import_w_collider;
use glengine::engine::drawable::material::{Material, MaterialData};
use glengine::engine::drawable::shader::lit::LIT_COLOR_SHADER;
use glengine::engine::scene::camera::Camera;
use glengine::engine::scene::gameobject::components::collider::ColliderComponent;
use glengine::engine::scene::gameobject::components::drawable::DrawableComponent;
use glengine::engine::scene::gameobject::components::rigidbody::RigidBodyComponent;
use glengine::engine::scene::gameobject::components::rotating::RotatingComponent;
use glengine::engine::scene::gameobject::GameObject;
use glengine::engine::scene::lights::directional::DirectionalLight;
use glengine::engine::scene::lights::point::PointLight;
use glengine::engine::scene::lights::spot::SpotLight;
use glengine::engine::scene::lights::Lights;
use glengine::engine::scene::Scene;
use glengine::engine::scene::SceneData;
use glengine::engine::transform::Transform;
use glengine::engine::Engine;
use glengine::engine::GameData;
use glengine::glam::{vec3, Mat4, Quat};
use glengine::result::EngineRenderResult;
use glengine::{gl, nmdl_import, nmdl_import_w_collider};
use rand::Rng;
use rapier3d::prelude::ColliderBuilder;
use rapier3d::prelude::RigidBodyBuilder;
use std::rc::Rc;

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

fn cube_rain(parent: Option<GameObject>, center: Vec3, size: Vec3, count: usize,restitution:f32) {
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        let x = rng.gen_range(center.x - size.x..center.x + size.x);
        let y = rng.gen_range(center.y - size.y..center.y + size.y);
        let z = rng.gen_range(center.z - size.z..center.z + size.z);
        let scale = rng.gen_range(1.0..5.0);
        let angvel = vec3(
            rng.gen_range(0.0..(4.0*std::f32::consts::PI)),
            rng.gen_range(0.0..(4.0*std::f32::consts::PI)),
            rng.gen_range(0.0..(4.0*std::f32::consts::PI)),
        );
        new_simulated_cube(parent.clone(), vec3(x, y, z), angvel, scale,restitution);
    }
}

fn new_simulated_cube(
    parent: Option<GameObject>,
    position: Vec3,
    angvel: Vec3,
    scale: f32,
    restitution: f32,
) -> GameObject {
    let cube = GameObject::new(parent);
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
        let mut data = cube.base.borrow_mut();
        data.add_component(DrawableComponent::new(Box::new(drawable)));
        data.data.transform.position = position;
        data.data.transform.scale *= scale;
        data.add_component(RigidBodyComponent::from(
            RigidBodyBuilder::dynamic().angvel(angvel.into()).build(),
        ));
        data.add_component(ColliderComponent::from(
            ColliderBuilder::cuboid(scale / 2.0, scale / 2.0, scale / 2.0)
                .restitution(restitution)
                .build(),
        ));
    }
    cube
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
        let empty = GameObject::new(None);
        self.data.objects.push(empty.clone());
        let monkey = GameObject::new(Some(empty.clone()));
        {
            let (monkey_draw, mut collider) = nmdl_import_w_collider!("monkeylp.obj", 5.0);
            collider.colliders[0].volume();
            let mut data = monkey.base.borrow_mut();
            data.data.transform.position = vec3(5.0, 10.0, 0.0);
            data.data.transform.scale *= 5.0;
            data.add_component(DrawableComponent::new(Box::new(monkey_draw)));
            data.add_component(RigidBodyComponent::from(
                RigidBodyBuilder::dynamic()
                    .angvel(vec3(1.0, 0.2, 0.0).into())
                    .build(),
            ));
            collider.colliders.iter_mut().for_each(|collider| {
                collider.set_restitution(0.8);
            });
            data.add_component(collider)
        }

        let floor = GameObject::new(Some(empty.clone()));
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
            let mut data = floor.base.borrow_mut();
            data.data.transform.scale *= 200.0;
            data.data.transform.scale.y *= 0.001;
            data.data.transform.position = vec3(0.0, -4.0, 0.0);
            data.add_component(DrawableComponent::new(Box::new(drawable)));
            data.add_component(RigidBodyComponent::from(RigidBodyBuilder::fixed().build()));
            data.add_component(ColliderComponent::from(
                ColliderBuilder::cuboid(100.0, 0.1, 100.0)
                    .restitution(0.2)
                    .build(),
            ));
        }

        cube_rain(
            Some(empty.clone()),
            vec3(0.0, 30.0, 0.0),
            vec3(20.0, 2.0, 20.0),
            20,
            0.7
        );

        // let cube = GameObject::new(Some(empty.clone()));
        // {
        //     let mut drawable = BaseDrawable::default();
        //     drawable.draw_data[0].shader = LIT_COLOR_SHADER.clone();
        //     drawable.draw_data[0].material = Some(Rc::new(Material {
        //         data: MaterialData {
        //             ambient: Some(vec3(0.9, 0.1, 0.1)),
        //             diffuse: Some(vec3(1.0, 0.4, 0.6)),
        //             specular: Some(vec3(1.0, 0.5, 0.7)),
        //             shininess: Some(0.02),
        //         },
        //         textures: Default::default(),
        //     }));
        //     let mut data = cube.base.borrow_mut();
        //     data.add_component(DrawableComponent::new(Box::new(drawable)));
        //     data.data.transform.scale *= 4.0;
        //     data.data.transform.position = vec3(0.0, -1.0, 8.0);
        //     data.add_component(RigidBodyComponent::from(RigidBodyBuilder::fixed().build()));
        //     data.add_component(ColliderComponent::from(ColliderBuilder::cuboid(4.0,4.0,4.0).restitution(0.1).build()));
        //
        // }

        let rotator = GameObject::new(Some(empty.clone()));
        {
            let mut data = rotator.base.borrow_mut();
            let drawable = nmdl_import!("bugatticlean.obj");
            data.add_component(DrawableComponent::new(Box::new(drawable)));
            data.add_component(RotatingComponent::new(vec3(0.0, 0.14, 0.0)));
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
        self.data_mut().lights.spot.push(spot_light);

        let point_light_gameobject = GameObject::new_w_transform(
            Some(empty.clone()),
            Transform::default().with_position(vec3(5.0, 13.0, -20.0)),
        );

        let point_light = PointLight::new_w_gameobject(
            point_light_gameobject.clone(),
            1.8,
            vec3(1.0, 1.0, 1.0),
            1.0,
            0.09,
            0.032,
        );
        self.data_mut().lights.point.push(point_light);

        let direction_light_gameobject = GameObject::new_w_transform(
            Some(empty.clone()),
            Mat4::from_quat(Quat::from_axis_angle(
                vec3(1.0, 0.6, 0.8),
                -std::f32::consts::FRAC_PI_2,
            ))
            .into(),
        );
        let directional_light = DirectionalLight::new_w_gameobject(
            direction_light_gameobject,
            0.04,
            vec3(1.0, 1.0, 1.0),
        );

        self.data_mut().lights.directional = Some(directional_light);

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
