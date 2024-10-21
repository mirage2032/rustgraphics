static USE_DISCRETE_GPU: bool = true;
fn main() {
    if USE_DISCRETE_GPU {
        #[cfg(target_os = "windows")]
        glengine::build_utils::gpu::use_discrete_gpu_win();
    }
}
