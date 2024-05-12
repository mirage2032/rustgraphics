use thiserror::Error;
use std::collections::HashMap;
use std::fmt;

pub type EngineRunResult = Result<(), EngineRunOut>;
pub type EngineRenderResult<T> = Result<T, EngineRenderError>;
pub type EngineStepResult<T> = Result<T, String>;

#[derive(Debug, Clone)]
pub struct EngineRunOut {
    pub main_result: Result<(),EngineRunError>,
    pub step_result: EngineStepResult<()>,
    pub render_result: EngineRenderResult<()>,
    
}

impl EngineRunOut {
    pub fn new() -> Self {
        Self {
            main_result: Ok(()),
            step_result: Ok(()),
            render_result: Ok(()),
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum ShaderError {
    #[error("Failed to read shadeer source")]
    ReadSourceError,
    #[error("Failed to compile shader")]
    CompileError,
    #[error("Failed to link shader program")]
    LinkError,
    #[error("Failed to create shader program")]
    CreateProgramError,
    #[error("Failed to create shader")]
    CreateShaderError,
}

#[derive(Error, Debug, Clone)]
pub enum EngineRenderError {
    #[error("Failed to create window")]
    CreateWindowError,
    #[error("Failed to initialize GLAD")]
    InitGlad,
    #[error("Shader error: {0}")]
    CreateShaderError(#[from] ShaderError),
    #[error("Could not join thread")]
    JoinThreadError,
}

#[derive(Error, Debug, Clone)]
pub enum EngineRunError {
    #[error("Engine rendering error:{0}")]
    RenderError(EngineRenderError),
    #[error("Engine stepping error:{0}")]
    StepError(String),
    #[error("Engine thread error:{0}")]
    ThreadError(String),
}