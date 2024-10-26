use std::path::{Path, PathBuf};
use std::{env, fs};
use syn::{visit::Visit, Macro};
use syn::__private::ToTokens;

static USE_DISCRETE_GPU: bool = false;

struct NmdlImportMacroFinder {
    // Store all the found macro invocations
    found_macros: Vec<PathBuf>,
}

impl<'ast> Visit<'ast> for NmdlImportMacroFinder {
    // Visit all macro invocations
    fn visit_macro(&mut self, mac: &'ast Macro) {
        let macro_name = mac.path.segments.last().unwrap().ident.to_string();
        let macro_names = ["nmdl_import", "nmdl_import_w_collider"];

        if macro_names.contains(&macro_name.as_str()) {
            // Attempt to extract the first argument
            if let Some(first_arg) = mac.tokens.clone().into_iter().next() {
                // Check if the first argument is a string literal
                if let Ok(lit_str) = syn::parse2::<syn::LitStr>(first_arg.into_token_stream()) {
                    // If it's a string literal, collect its value
                    self.found_macros.push(lit_str.value().into());
                }
            }
        }
        // Continue visiting the rest of the syntax tree
        syn::visit::visit_macro(self, mac);
    }
}
fn get_nmdl_imports() -> Vec<PathBuf> {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = Path::new(&crate_dir).join("src");
    let mut imports = Vec::new();
    for entry in glob::glob(&format!("{}/**/*.rs", src_dir.to_str().unwrap())).unwrap() {
        match entry {
            Ok(path) => {
                let content = fs::read_to_string(&path).unwrap();
                let syntax = syn::parse_file(&content).unwrap();
                let mut finder = NmdlImportMacroFinder {
                    found_macros: Vec::new(),
                };
                finder.visit_file(&syntax);
                imports.extend(finder.found_macros);
            }
            Err(e) => println!("{:?}", e),
        }
    }
    imports
}
fn main() {
    if USE_DISCRETE_GPU {
        #[cfg(target_os = "windows")]
        glengine::build_utils::gpu::use_discrete_gpu_win();
    }
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rerun-if-changed={} ", Path::new(&crate_dir).join("src").to_str().unwrap());

    let models_in = Path::new(&crate_dir).join("models");
    let out_dir = env::var("OUT_DIR").unwrap();
    let models_out = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap().join("models");
    let mut to_import = get_nmdl_imports();
    match glengine::build_utils::models::with_convert_dir(&models_in, &models_out,|path,destination|{
        //check if path is newer than destination
        let relative = path.strip_prefix(&models_in).unwrap();
        let mut should_import = to_import.iter().any(|import| import == relative);
        if should_import { 
            if let (Ok(path_meta),Ok(dest_meta)) = (fs::metadata(path),fs::metadata(destination)) {
                if path_meta.modified().unwrap() < dest_meta.modified().unwrap() {
                    should_import = false; 
                }
            }
            to_import.retain(|import| import != relative);
        }
        should_import
    }){
        Ok(files) =>{
            for (src,_) in files{
                println!("cargo:rerun-if-changed={} ", src.to_str().unwrap());
            }
        }
        Err(err) => {
            panic!("Failed to convert models. {}",err);
        }
    }
    if !to_import.is_empty(){
        panic!("Some files used in nmdl_import! were nout found: {:?}", to_import);
    }
}
