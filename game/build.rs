use std::path::Path;

static USE_DISCRETE_GPU: bool = true;
fn main() {
    if USE_DISCRETE_GPU {
        #[cfg(target_os = "windows")]
        glengine::build_utils::gpu::use_discrete_gpu_win();
    }
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let models_in = Path::new(&crate_dir).join("models");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let models_out = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap().join("models");
    match glengine::build_utils::models::convert_dir(&models_in, &models_out){
        Ok(files) =>{
            for file in files{
                println!("cargo:rerun-if-changed={}", file.to_str().unwrap());
            }
        }
        Err(err) => {
            panic!("Failed to convert models");
        }
    }
}
