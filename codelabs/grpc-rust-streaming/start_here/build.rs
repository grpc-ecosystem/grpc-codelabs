use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let proto_path = Path::new("proto/routeguide.proto");
    let generated_dir = Path::new("generated");
    let gen_marker = Path::new("generated/generated.rs");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", proto_path.display());
    println!("cargo:rerun-if-changed={}", gen_marker.display());
    println!("cargo:rerun-if-env-changed=GRPC_RUST_REGENERATE_PROTO");

    let force_regenerate = env::var_os("GRPC_RUST_REGENERATE_PROTO").is_some();
    let generated_missing = !generated_dir.exists() || !gen_marker.exists();
    let proto_newer = match (fs::metadata(proto_path), fs::metadata(gen_marker)) {
        (Ok(p_meta), Ok(g_meta)) => match (p_meta.modified(), g_meta.modified()) {
            (Ok(p_time), Ok(g_time)) => p_time > g_time,
            _ => true,
        },
        _ => true,
    };

    if force_regenerate || generated_missing || proto_newer {
        if generated_dir.exists() {
            let _ = fs::remove_dir_all(generated_dir);
        }

        grpc_protobuf_build::CodeGen::new()
            .output_dir(generated_dir)
            .include("proto")
            .input("routeguide.proto")
            .compile()
            .unwrap();
    }
}
