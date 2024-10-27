use std::cell::RefCell;
use once_cell::unsync::Lazy;
use crate::engine::drawable::material::manager::MaterialManager;
use crate::engine::drawable::mesh::manager::MeshManager;
use crate::engine::drawable::shader::manager::ShaderManager;

pub struct DrawableManager {
    pub mesh: MeshManager,
    pub material: MaterialManager,
    pub shader: ShaderManager,
}

impl DrawableManager {
    pub fn clean(&mut self) {
        self.mesh.clean();
        self.material.clean();
        self.shader.clean();
    }
}

impl Default for DrawableManager {
    fn default() -> Self {
        Self {
            mesh: MeshManager::default(),
            material: MaterialManager::default(),
            shader: ShaderManager::default(),
        }
    }
}

thread_local! {
    pub static DRAWABLE_MANAGER: Lazy<RefCell<DrawableManager>> = Lazy::new(|| RefCell::new(DrawableManager::default()));
}