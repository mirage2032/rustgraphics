pub mod gpu {
    #[no_mangle]
    pub static NvOptimusEnablement: i32 = 1;
    #[no_mangle]
    pub static AmdPowerXpressRequestHighPerformance: i32 = 1;
    pub fn use_nvidia_optimus() {
        println!("cargo:rustc-link-arg=/EXPORT:NvOptimusEnablement");
    }

    pub fn use_amd_high_perf() {
        println!("cargo:rustc-link-arg=/EXPORT:AmdPowerXpressRequestHighPerformance");
    }

    pub fn use_discrete_gpu_win() {
        use_nvidia_optimus();
        use_amd_high_perf();
    }
}

pub mod models{
    use std::error::Error;
    use std::path::{Path, PathBuf};
    use glengine_mdl::models::{FileStruct, EXTENSION};
    const CONVERTIBLE_EXTENSIONS: [&str;1] = ["obj"];
    pub fn convert_file(source: &Path,destination:&Path) -> Result<(),Box<dyn Error>> {
        let file = FileStruct::import(source.to_str().unwrap());
        std::fs::create_dir_all(Path::new(&destination).parent().unwrap())?;
        file.save(destination.to_str().unwrap())
    }

    pub fn convert_name(file:&str,source: &str,destination:&str) -> PathBuf {
        Path::new(&file.replace(source, destination)).with_extension(EXTENSION)
    }
    pub fn convert_dir(source: &Path,destination:&Path) -> Result<Vec<PathBuf>,Box<dyn Error>> {
        let mut converted = Vec::new();
        for entry in glob::glob(&format!("{}/**/*", source.to_str().unwrap()))? {
            match entry {
                Ok(path) => {
                    if !CONVERTIBLE_EXTENSIONS.iter().any(|ext| path.to_str().unwrap().ends_with(ext)){
                        continue;
                    }
                    let destination = convert_name(path.to_str().unwrap(),source.to_str().unwrap(),destination.to_str().unwrap());
                    convert_file(&path,&destination)?;
                    converted.push(destination);
                }
                Err(e) => println!("{:?}", e),
            }
        }
        Ok(converted)
    }
}
