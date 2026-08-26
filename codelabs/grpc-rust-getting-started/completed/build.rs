fn main() {
    grpc_protobuf_build::CodeGen::new()
        .include("proto")
        .input("routeguide.proto")
        .output_dir("generated")
        .compile()
        .unwrap();
}
