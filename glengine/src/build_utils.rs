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

    pub fn convert_name(file:&Path,source: &Path,destination:&Path) -> PathBuf {
        // add EXTENSION to end of filename, keeping old extension before new one
        let mut dest = destination.join(file.strip_prefix(source).unwrap());
        //add ".nmdl" to end of filename but not replace old extension
        dest.with_extension(dest.extension().unwrap().to_str().unwrap().to_string() + "." + EXTENSION)
    }
    pub fn convert_dir(source: &Path,destination:&Path) -> Result<Vec<PathBuf>,Box<dyn Error>> {
        let mut converted = Vec::new();
        for entry in glob::glob(&format!("{}/**/*", source.to_str().unwrap()))? {
            match entry {
                Ok(path) => {
                    if !CONVERTIBLE_EXTENSIONS.iter().any(|ext| path.to_str().unwrap().ends_with(ext)){
                        continue;
                    }
                    let destination = convert_name(&path,&source,&destination);
                    convert_file(&path,&destination)?;
                    converted.push(path);
                }
                Err(e) => println!("{:?}", e),
            }
        }
        Ok(converted)
    }
    pub fn with_convert_dir<C: FnMut(&Path)->bool>(source: &Path, destination:&Path,mut should_convert:C) -> Result<Vec<PathBuf>,Box<dyn Error>> {
        let mut converted = Vec::new();
        for entry in glob::glob(&format!("{}/**/*", source.to_str().unwrap()))? {
            match entry {
                Ok(path) => {
                    if !should_convert(&path){
                        continue;
                    }
                    if !CONVERTIBLE_EXTENSIONS.iter().any(|ext| path.to_str().unwrap().ends_with(ext)) {
                        continue;
                    }
                    let destination = convert_name(&path,&source,&destination);
                    convert_file(&path,&destination)?;
                    converted.push(path);
                }
                Err(e) => println!("{:?}", e),
            }
        }
        Ok(converted)
    }
}
