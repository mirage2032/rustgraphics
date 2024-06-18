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
