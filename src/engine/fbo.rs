use gl::types::GLuint;

pub struct Fbo {
    pub fbo: GLuint,
    pub texture: GLuint,
    pub depth_texture: GLuint,
    pub width: u32,
    pub height: u32,
}

impl Fbo{
    pub fn new(width: u32, height: u32) -> Self {
        let mut fbo = 0;
        let mut texture = 0;
        let mut depth_texture = 0;
        let mut stencil_buffer = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            // Color texture
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA16F as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::FLOAT,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture,
                0,
            );
            
            // Depth and Stencil texture
            gl::GenTextures(1, &mut depth_texture);
            gl::BindTexture(gl::TEXTURE_2D, depth_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::DEPTH24_STENCIL8 as i32,
                width as i32,
                height as i32,
                0,
                gl::DEPTH_STENCIL,
                gl::UNSIGNED_INT_24_8,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_STENCIL_ATTACHMENT,
                gl::TEXTURE_2D,
                depth_texture,
                0,
            );

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete");
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        Self {
            fbo,
            texture,
            depth_texture,
            width,
            height,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl Drop for Fbo {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo);
            gl::DeleteTextures(1, &self.texture);
            gl::DeleteRenderbuffers(1, &self.depth_texture);
        }
    }
}