use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};
use once_cell::unsync::Lazy;
use crate::engine::drawable::shader::{lit, unlit, Shader};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncludedShaderHandle {
    Basic,
    LitColor,
    UnlitFace,
    UnlitQuad
}

#[derive(Clone)]
pub enum ShaderHandle{
    Included(IncludedShaderHandle),
    Custom(CustomShaderHandle)
}

impl From<IncludedShaderHandle> for ShaderHandle{
    fn from(handle: IncludedShaderHandle) -> Self{
        ShaderHandle::Included(handle)
    }
}

impl From<CustomShaderHandle> for ShaderHandle{
    fn from(handle: CustomShaderHandle) -> Self{
        ShaderHandle::Custom(handle)
    }
}

#[derive(Clone)]
pub struct CustomShaderHandle{
    rc: Rc<()>,
    handle:usize
}

#[derive(Clone)]
struct CustomShaderWeakHandle{
    weak: Weak<()>
}

pub struct ShaderManager {
    included: HashMap<IncludedShaderHandle, Shader>,
    custom: HashMap<usize, (Shader,CustomShaderWeakHandle)>,
    custom_index: usize
}

impl ShaderManager {

    pub fn get_included(&self, included: &IncludedShaderHandle) -> &Shader{
        self.included.get(&included).unwrap()
    }

    pub fn get_included_mut(&mut self, included: &IncludedShaderHandle) -> &mut Shader{
        self.included.get_mut(&included).unwrap()
    }
    pub fn get(&self, shader_type: &ShaderHandle) -> Option<&Shader>{
        match shader_type{
            ShaderHandle::Included(included) => Some(self.get_included(included)),
            ShaderHandle::Custom(custom) => self.custom.get(&custom.handle).map(|(shader,_)|shader)
        }
    }
    pub fn get_mut(&mut self, shader_type: &ShaderHandle) -> Option<&mut Shader>{
        match shader_type{
            ShaderHandle::Included(included) => Some(self.get_included_mut(included)),
            ShaderHandle::Custom(custom) => self.custom.get_mut(&custom.handle).map(|(shader,_)|shader)
        }
    }

    pub fn get_custom(&self, handle: &CustomShaderHandle) -> Option<&Shader>{
        self.custom.get(&handle.handle).map(|(shader,_)|shader)
    }

    pub fn get_custom_mut(&mut self, handle: &CustomShaderHandle) -> Option<&mut Shader>{
        self.custom.get_mut(&handle.handle).map(|(shader,_)|shader)
    }
    pub fn add(&mut self, shader: Shader) -> CustomShaderHandle{
        let index = self.custom_index;
        let handle = CustomShaderHandle{
            rc: Rc::new(()),
            handle: index
        };
        let weak = CustomShaderWeakHandle{
            weak: Rc::downgrade(&handle.rc)
        };
        self.custom.insert(index, (shader,weak));
        self.custom_index += 1;
        while let Some((_,weak)) = self.custom.get(&self.custom_index){
            match weak.weak.upgrade(){
                Some(_) => self.custom_index += 1,
                None => {
                    self.custom.remove(&self.custom_index);
                    break;
                }
            }
        }
        handle
    }

    pub fn clean(&mut self){
        self.custom.retain(|_,(_,weak)|weak.weak.upgrade().is_some());
    }

    pub fn remove(&mut self, index: usize){
        self.custom.remove(&index);
    }
}

impl Default for ShaderManager {
    fn default() -> Self{
        let mut included = HashMap::new();
        included.insert(IncludedShaderHandle::Basic, Shader::default());
        included.insert(IncludedShaderHandle::LitColor, lit::new_basic_shader().unwrap());
        included.insert(IncludedShaderHandle::UnlitFace, unlit::new_face_shader().unwrap());
        included.insert(IncludedShaderHandle::UnlitQuad, unlit::new_quad_shader().unwrap());
        Self{
            included,
            custom: HashMap::new(),
            custom_index: 0
        }
    }
}