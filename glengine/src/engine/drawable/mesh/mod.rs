use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use once_cell::unsync::Lazy;

pub mod cube;
pub mod screenquad;

pub trait Mesh: Send + Sync {
    fn get(&self) -> &MeshData;
    fn get_mut(&mut self) -> &mut MeshData;
    fn bind(&self);
    fn draw(&self);
}

pub struct BaseMesh {
    pub mesh_data: MeshData,
}

impl Mesh for BaseMesh {
    fn get(&self) -> &MeshData {
        &self.mesh_data
    }
    fn get_mut(&mut self) -> &mut MeshData {
        &mut self.mesh_data
    }
    fn bind(&self) {
        self.mesh_data.bind();
    }
    fn draw(&self) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.mesh_data.indices_count as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}

pub struct MeshData {
    vao: u32,
    vbo_vertices: u32,
    vbo_normals: Option<u32>,
    vbo_texcoords: Option<u32>,
    ebo: Option<u32>,
    indices_count: u32,
}

impl MeshData {
    pub fn new(vertices: &[f32]) -> Self {
        let vao = unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            vao
        };

        let vbo_vertices = unsafe {
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            vbo
        };
        unsafe {
            gl::BindVertexArray(0);
        }
        Self {
            vao,
            vbo_vertices,
            vbo_normals: None,
            vbo_texcoords: None,
            ebo: None,
            indices_count: vertices.len() as u32 / 3,
        }
    }
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn with_normals(mut self, normals: &[f32]) -> Self {
        self.bind();
        self.vbo_normals = unsafe {
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (normals.len() * std::mem::size_of::<f32>()) as isize,
                normals.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1);
            Some(vbo)
        };
        Self::unbind();
        self
    }

    pub fn with_texcoords(mut self, texcoords: &[f32]) -> Self {
        self.bind();
        self.vbo_texcoords = unsafe {
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (texcoords.len() * std::mem::size_of::<f32>()) as isize,
                texcoords.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(2);
            Some(vbo)
        };
        Self::unbind();
        self
    }

    pub fn with_indices(mut self, indices: &[u32]) -> Self {
        self.bind();
        self.ebo = unsafe {
            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
            Some(ebo)
        };
        self.indices_count = indices.len() as u32;
        Self::unbind();
        self
    }
    pub fn get_indices_count(&self) -> u32 {
        self.indices_count
    }
    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for MeshData {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo_vertices);
            if let Some(vbo) = self.vbo_normals {
                gl::DeleteBuffers(1, &vbo);
            };
            if let Some(vbo) = self.vbo_texcoords {
                gl::DeleteBuffers(1, &vbo);
            };
            if let Some(ebo) = self.ebo {
                gl::DeleteBuffers(1, &ebo);
            };
        }
    }
}

#[derive(Clone)]
pub struct MeshHandle{
    rc: Rc<()>,
    handle:usize
}

#[derive(Clone)]
struct MeshWeakHandle{
    handle: Weak<()>
}

pub struct MeshMap{
    meshes: HashMap<usize, (Box<dyn Mesh>,MeshWeakHandle)>,
    index: usize
}

impl MeshMap{
    pub fn get(&self, handle: &MeshHandle) -> Option<&Box<dyn Mesh>>{
        let (mesh,_) = self.meshes.get(&handle.handle)?;
        Some(mesh)
    }
    pub fn get_mut(&mut self, handle: &MeshHandle) -> Option<&mut Box<dyn Mesh>>{
        let (mesh,_) = self.meshes.get_mut(&handle.handle)?;
        Some(mesh)
    }
    pub fn add(&mut self, mesh: Box<dyn Mesh>) -> MeshHandle{
        let index = self.index;
        let mesh_handle = MeshHandle{
            rc: Rc::new(()),
            handle: index
        };
        let weak_handle = MeshWeakHandle{
            handle: Rc::downgrade(&mesh_handle.rc)
        };
        self.meshes.insert(index, (mesh,weak_handle));
        
        self.index = self.index.wrapping_add(1);
        while let Some((_,weak)) = self.meshes.get(&self.index){
            match weak.handle.upgrade(){
                Some(_) => self.index = self.index.wrapping_add(1),
                None => {
                    self.meshes.remove(&self.index);
                    break;
                }
            }
        }
        mesh_handle
    }
    
    pub fn clean(&mut self){
        self.meshes.retain(|_,(_,weak)|weak.handle.upgrade().is_some());
    }
    pub fn remove(&mut self, handle: MeshHandle){
        self.meshes.remove(&handle.handle);
    }
}

impl Default for MeshMap{
    fn default() -> Self{
        Self{
            meshes: HashMap::new(),
            index: 0
        }
    }
}

thread_local! {
    pub static MESH_MAP: Lazy<RefCell<MeshMap>> = Lazy::new(|| RefCell::new(MeshMap::default()));
}
