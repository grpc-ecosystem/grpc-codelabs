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
    println!("cargo:rerun-if-env-changed=SKIP_GRPC_RUST_PROTO_CODEGEN");

    let skip_codegen = env::var_os("SKIP_GRPC_RUST_PROTO_CODEGEN").is_some();
    let force_regenerate = env::var_os("GRPC_RUST_REGENERATE_PROTO").is_some();

    if skip_codegen {
        if force_regenerate {
            println!(
                "cargo:warning=Both SKIP_GRPC_RUST_PROTO_CODEGEN and GRPC_RUST_REGENERATE_PROTO are set. Skipping code generation."
            );
        }
        assert!(
            gen_marker.exists(),
            "SKIP_GRPC_RUST_PROTO_CODEGEN is set, but generated files are missing at {}",
            gen_marker.display()
        );
        return;
    }

    let generated_missing = !generated_dir.exists() || !gen_marker.exists();
    let proto_newer = match (fs::metadata(proto_path), fs::metadata(gen_marker)) {
        (Ok(p_meta), Ok(g_meta)) => match (p_meta.modified(), g_meta.modified()) {
            (Ok(p_time), Ok(g_time)) => p_time > g_time,
            _ => true,
        },
        _ => true,
    };

    if force_regenerate || generated_missing || proto_newer {
        if has_protoc() {
            if generated_dir.exists() {
                let _ = fs::remove_dir_all(generated_dir);
            }

            grpc_protobuf_build::CodeGen::new()
                .output_dir(generated_dir)
                .include("proto")
                .input("routeguide.proto")
                .compile()
                .unwrap();
        } else if generated_missing {
            panic!(
                "Cannot generate protobuf code: protoc is not available and generated files are missing at {}",
                generated_dir.display()
            );
        } else {
            println!(
                "cargo:warning=protoc not found; skipping proto regeneration and using checked-in files."
            );
        }
    }
}

fn has_protoc() -> bool {
    if env::var_os("GRPC_RUST_PROTOC_DIR").is_some_and(|dir| {
        !dir.is_empty()
            && (Path::new(&dir).join("protoc").is_file()
                || Path::new(&dir).join("protoc.exe").is_file())
    }) {
        return true;
    }
    if env::var_os("PROTOC").is_some_and(|p| !p.is_empty() && Path::new(&p).is_file()) {
        return true;
    }
    if let Some(path_var) = env::var_os("PATH") {
        for dir in env::split_paths(&path_var) {
            if dir.join("protoc").is_file() || dir.join("protoc.exe").is_file() {
                return true;
            }
        }
    }
    false
}
