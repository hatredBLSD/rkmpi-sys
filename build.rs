use std::env;

fn main() {
    if let Ok(lib_dir) = env::var("ROCKIT_LIB") {
        println!("cargo:rustc-link-search=native={lib_dir}");
    }
    println!("cargo:rustc-link-lib=dylib=rockit");
    println!("cargo:rerun-if-env-changed=ROCKIT_LIB");
}