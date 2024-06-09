static USE_DISCRETE_GPU: bool = true;
fn main() {
    if USE_DISCRETE_GPU {
        glengine::build_utils::GPU::use_discrete_gpu();
    }
}
