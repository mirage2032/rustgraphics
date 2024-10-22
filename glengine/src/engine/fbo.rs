use crate::engine::drawable;
use crate::engine::drawable::DrawData;
use crate::engine::scene::Scene;
use gl::types::GLuint;

pub struct Fbo {
    pub fbo: GLuint,
    pub resolved_fbo: GLuint,
    pub color_rbo: GLuint,
    pub color_texture: GLuint,
    pub depth_stencil_rbo: GLuint,
    pub depth_stencil_texture: GLuint,
    pub width: u32,
    pub height: u32,
}

impl Fbo{
    pub fn new(width: u32, height: u32,samples:i32) -> Self {
        let mut fbo = 0;
        let mut resolved_fbo = 0;
        let mut color_rbo = 0;
        let mut color_texture = 0;
        let mut depth_stencil_rbo = 0;
        let mut depth_stencil_texture = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut resolved_fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, resolved_fbo);
            gl::GenTextures(1, &mut color_texture);
            gl::BindTexture(gl::TEXTURE_2D, color_texture);
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
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, color_texture, 0);

            gl::GenTextures(1, &mut depth_stencil_texture);
            gl::BindTexture(gl::TEXTURE_2D, depth_stencil_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::DEPTH32F_STENCIL8 as i32,
                width as i32,
                height as i32,
                0,
                gl::DEPTH_STENCIL,
                gl::FLOAT_32_UNSIGNED_INT_24_8_REV,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::TEXTURE_2D, depth_stencil_texture, 0);
            // 
            // gl::GenTextures(1, &mut stencil_texture);
            // gl::BindTexture(gl::TEXTURE_2D, stencil_texture);
            // gl::TexImage2D(
            //     gl::TEXTURE_2D,
            //     0,
            //     gl::STENCIL_INDEX8 as i32,
            //     width as i32,
            //     height as i32,
            //     0,
            //     gl::STENCIL_INDEX,
            //     gl::UNSIGNED_BYTE,
            //     std::ptr::null(),
            // );
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            // gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::STENCIL_ATTACHMENT, gl::TEXTURE_2D, stencil_texture, 0);

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete");
            }            
            
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            // Color renderbuffer
            gl::GenRenderbuffers(1, &mut color_rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, color_rbo);
            gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, samples, gl::RGBA16F, width as i32, height as i32);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER, color_rbo);

            // Depth framebuffer
            gl::GenRenderbuffers(1, &mut depth_stencil_rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, depth_stencil_rbo);
            gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, samples, gl::DEPTH32F_STENCIL8, width as i32, height as i32);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, depth_stencil_rbo);

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete");
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        Self {
            fbo,
            resolved_fbo,
            color_rbo,
            color_texture,
            depth_stencil_rbo,
            depth_stencil_texture,
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

    pub fn blit(&self){
        unsafe {
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.resolved_fbo);
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.fbo);
            gl::BlitFramebuffer(
                0,
                0,
                self.width as i32,
                self.height as i32,
                0,
                0,
                self.width as i32,
                self.height as i32,
                gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT,
                gl::NEAREST,
            );
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
            gl::DeleteFramebuffers(1, &self.resolved_fbo);
            gl::DeleteFramebuffers(1, &self.fbo);
            gl::DeleteRenderbuffers(1, &self.color_rbo);
            gl::DeleteRenderbuffers(1, &self.depth_stencil_rbo);
            gl::DeleteTextures(1, &self.color_texture);
            gl::DeleteTextures(1, &self.depth_stencil_texture);
        }
    }
}

pub struct ScreenFbo {
    pub fbo:Fbo,
    pub draw_data:DrawData
}
impl ScreenFbo{
    pub fn new(width:u32,height:u32,samples:i32)->ScreenFbo{
        let fbo = Fbo::new(width, height,samples); // Limit multi-sampling to supported max samples
        let draw_data = drawable::screenquad(&fbo);
        ScreenFbo{
            fbo,
            draw_data
        }
    }
    pub fn render(&self, scene:&mut Box<dyn Scene>) {
        unsafe {
            self.fbo.bind();
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            scene.render();
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.fbo.fbo);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
            self.fbo.blit();
            Fbo::unbind();
        }
    }
}