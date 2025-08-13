# Generate Code from Proto Files

This is a tutorial for generating code from Proto files using gRPC-Rust. This tutorial will utilize the RouteGuide example.

### Prerequisites

* [**Tonic**](https://github.com/hyperium/tonic.git), the open source repository that gRPC-Rust is build off on
```sh
$ git clone https://github.com/hyperium/tonic.git
```
* [**Rust**](https://www.rust-lang.org/).
    * Follow installation instructions [here](https://www.rust-lang.org/tools/install).
* [**Bazel 8.3.1**](https://bazel.build/).
    * Follow installation instructions [here](https://github.com/bazelbuild/bazel/releases).
* [**Protocol buffer**](https://developers.google.com/protocol-buffers) **compiler**, `protoc`, [version 3](https://protobuf.dev/programming-guides/proto3).
    * For installation instructions, see [Protocol Buffer Compiler Installation](https://grpc.io/docs/protoc-installation/).
    * NOTE: Must need a version of Protoc 3.31.1 or higher.
* **Rust plugins** for the protocol compiler:
```sh
$ cd tonic/protoc-gen-rust-grpc
$ bazel build //src:protoc-gen-rust-grpc
$ PLUGIN_PATH="$(pwd)/bazel-bin/src/protoc-gen-rust-grpc"
```

* Update your PATH so that the protoc compiler can find the plugins:

```sh
export PATH="$(pwd)/bazel-bin/src/:$PATH"
```

## Generating client and server code

Next we need to generate the gRPC client and server interfaces from our `.proto`
service definition. 

### Dependencies
Edit `Cargo.toml` and add the dependency we'll need for this example, which is tonic-protobuf-build:

```console
cargo add tonic-protobuf-build
```

### Compiling and Building Proto  
Create a `build.rs` file at the root of your crate. A build.rs script is a Rust program that Cargo executes before compiling your main project. Its purpose is to perform tasks like generating source code, linking to non-Rust libraries, or setting environment variables that influence the build process.

In this case, we will be putting the command to compile and build the `.proto` file in build.rs. We will use gRPC's tonic_protobuf_build crate to generate code from the `.proto` file.
```rust
fn main() {
    tonic_protobuf_build::CodeGen::new()
    .include("proto")
    .inputs(["routeguide.proto"])
    .output_dir("generated")
    .compile()
    .unwrap();
}
```
Now, run 
```shell
$ cargo build
```

That's it. The generated code contains:

- Struct definitions for message types `Point` and `Feature`.
- A service trait we'll need to implement: `route_guide_server::RouteGuide`.
- A client type we'll use to call the server: `route_guide_client::RouteGuideClient<T>`.

If your are curious as to where the generated files are, keep reading. The mystery will be revealed
soon! We can now move on to the fun part.

## Bringing Generated Code into Scope

The generated code is placed inside our target directory, in a location defined by the `OUT_DIR`
environment variable that is set by cargo. For our example, this means you can find the generated
code in a path similar to `target/debug/build/routeguide/out/routeguide.rs`.

We can use gRPC's `include_proto` macro to bring the generated code into scope:

```rust
pub mod routeguide {
    tonic::include_proto!("routeguide");
}
```

**Note**: The token passed to the `include_proto` macro (in our case "routeguide") is the name of
the package declared in our `.proto` file, not a filename, e.g "routeguide.rs".

With this in place, we can stub out our service implementation:

