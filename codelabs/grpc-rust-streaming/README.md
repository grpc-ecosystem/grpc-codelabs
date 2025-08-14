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
Edit `Cargo.toml` and add all the dependencies we'll need for this example:

```toml
[package]
edition = "2021"
license = "MIT"
name = "getting-started"

[[bin]]
name = "routeguide-server"
path = "src/server/server.rs"

[[bin]]
name = "routeguide-client"
path = "src/client/client.rs"

[features]
routeguide = ["dep:async-stream", "dep:tokio-stream", "dep:rand", "dep:serde", "dep:serde_json"]
full = ["routeguide"]
default = ["full"]

[dependencies]
# Common dependencies
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
prost = "0.14"
tonic = { git = "https://github.com/hyperium/tonic", branch="master"}
tonic-protobuf = {git = "https://github.com/hyperium/tonic", branch = "master", package = "tonic-protobuf" }
grpc = {git = "https://github.com/hyperium/tonic", branch = "master", package = "grpc"}
tonic-prost = {git = "https://github.com/hyperium/tonic", branch = "master", package = "tonic-prost" }
# Optional dependencies
async-stream = { version = "0.3", optional = true }
tokio-stream = { version = "0.1", optional = true }
tokio-util = { version = "0.7.8", optional = true }
tower = { version = "0.5", optional = true }
rand = { version = "0.9", optional = true }
serde = { version = "1.0", features = ["derive"], optional = true }
serde_json = { version = "1.0", optional = true }
prost-types = { version = "0.14", optional = true }
http = { version = "1", optional = true }
hyper = { version = "1", optional = true }
hyper-util = { version = "0.1.4", optional = true }
tokio-rustls = { version = "0.26.1", optional = true, features = ["ring", "tls12"], default-features = false }
hyper-rustls = { version = "0.27.0", features = ["http2", "ring", "tls12"], optional = true, default-features = false }
tower-http = { version = "0.6", optional = true }
protobuf = { version = "4.31.1-release"}

[build-dependencies]
tonic-protobuf-build = {git = "https://github.com/hyperium/tonic.git", branch = "master", package = "tonic-protobuf-build" }
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

## Bringing Generated Code into Scope

The generated code is placed inside our target directory, in a location defined by the `OUT_DIR`
environment variable that is set by cargo. For our example, this means you can find the generated
code in a path similar to `target/debug/build/routeguide/out/routeguide.rs`.

We can use gRPC's `include_proto` macro to bring the generated code into scope:

```rust
pub mod routeguide {
    grpc::include_proto!("routeguide");
}
```

**Note**: The token passed to the `include_proto` macro (in our case "routeguide") is the name of
the package declared in our `.proto` file, not a filename, e.g "routeguide.rs".

With this in place, we can stub out our service implementation:



