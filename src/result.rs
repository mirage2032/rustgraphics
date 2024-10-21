use thiserror::Error;

pub type EngineRunResult = Result<(), EngineRunError>;
pub type EngineRenderResult<T> = Result<T, EngineRenderError>;
pub type EngineStepResult<T> = Result<T, String>;
pub type EngineFixedStepResult<T> = Result<T, String>;

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
    RenderError(#[from] EngineRenderError),
    #[error("Engine stepping error:{0}")]
    StepError(String),
    #[error("Engine fixed stepping error:{0}")]
    FixedStepError(String),
    #[error("Engine import error:{0}")]
    ImportError(String),
    #[error("Engine thread error:{0}")]
    ThreadError(String),
}
