fn main() {
    protobuf_codegen::CodeGen::new()
        .include("src/routeguide")
        .inputs(["routeguide.proto"])
        .output_dir("generated")
        .compile_only()
        .unwrap();
}