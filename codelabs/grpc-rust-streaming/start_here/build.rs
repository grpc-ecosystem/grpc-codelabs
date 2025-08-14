fn main() {
    protobuf_codegen::CodeGen::new()
        .include("proto")
        .inputs(["routeguide.proto"])
        .output_dir("generated")
        .compile_only()
        .unwrap();
}