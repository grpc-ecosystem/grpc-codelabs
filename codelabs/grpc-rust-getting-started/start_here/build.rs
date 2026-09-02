use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=GRPC_RUST_REGENERATE_PROTO");

    if env::var_os("GRPC_RUST_REGENERATE_PROTO").is_some() {
        let generated_dir = Path::new("generated");
        if generated_dir.exists() {
            fs::remove_dir_all(generated_dir)
                .expect("All files in generated/ directory should be deletable");
        }

        grpc_protobuf_build::CodeGen::new()
            .output_dir(generated_dir)
            .include("proto")
            .input("routeguide.proto")
            .compile()
            .unwrap();
    }
}
