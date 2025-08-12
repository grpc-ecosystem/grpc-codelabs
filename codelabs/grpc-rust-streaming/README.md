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
## Defining protobuf messages and services

Our first step is to define the gRPC *service* and the method *request* and
*response* types using [protocol buffers](https://protobuf.dev/overview).

Let’s start by defining the messages and service in [this file](start_here/routeguide/route_guide.proto).

### Define the proto Messages

Our `.proto` file contains protocol buffer message type definitions for all the
request and response types used in our service methods.

Let’s define the `Point` message type.  `Point` are represented as
latitude-longitude pairs in the E7 representation.  For the purpose of this
codelabs, we will be using `integer` to define latitude and longitude.

```proto
message Point {
  int32 latitude = 1;
  int32 longitude = 2;
}
```

Let’s also define the `Feature` message type. A `Feature` names something at a
given point using a `string` field.

```proto
message Feature {
  // The name of the feature.
  string name = 1;

  // The point where the feature is detected.
  Point location = 2;
}
```

Next a `Rectangle` message which represents a latitude-longitude rectangle,
represented as two diagonally opposite points "lo" and "hi".

```proto
message Rectangle {
  // One corner of the rectangle.
  Point lo = 1;

  // The other corner of the rectangle.
  Point hi = 2;
}
```

Also a `RouteNote` message which represents a message sent while at a given
point.

```proto
message RouteNote {
  // The location from which the message is sent.
  Point location = 1;

  // The message to be sent.
  string message = 2;
}
```

We would also require a `RouteSummary` message.  This message is receieved in
response to a `RecordRoute` rpc which is explained in the next section.  It
contains the number of individual points received, the number of detected
features, and the total distance covered as the cumulative sum of the distance
between each point.

```proto
message RouteSummary {
  // The number of points received.
  int32 point_count = 1;

  // The number of known features passed while traversing the route.
  int32 feature_count = 2;

  // The distance covered in metres.
  int32 distance = 3;

  // The duration of the traversal in seconds.
  int32 elapsed_time = 4;
}
```

### Define the RouteGuide service

To define a service, you specify a named service in your `.proto` file:

```proto
service RouteGuide {
  // Definition of the service goes here
}
```

### Define the RPC method in the service

Then you define `rpc` methods inside your service definition, specifying their
request and response types.  In this section of the codelab, let’s define:

#### ListFeatures

Obtains the Features available within the given Rectangle. Results are streamed
rather than returned at once (e.g. in a response message with a repeated field),
as the rectangle may cover a large area and contain a huge number of features.

An appropriate type for this RPC is *server-side* streaming RPC. A server-side
streaming RPC where the client sends a request to the server and gets a stream
to read a sequence of messages back. The client reads from the returned stream
until there are no more messages. As you can see in our example, you specify a
server-side streaming method by placing the stream keyword before the response
type.

```proto
rpc ListFeatures(Rectangle) returns (stream Feature) {}
```

#### RecordRoute

Accepts a stream of Points on a route being traversed, returning a `RouteSummary`
when traversal is completed.

A *client-side streaming* RPC seems appropriate in this case.  A client-side
streaming RPC where the client writes a sequence of messages and sends them to
the server, again using a provided stream. Once the client has finished writing
the messages, it waits for the server to read them all and return its response.
You specify a client-side streaming method by placing the stream keyword before
the request type. `GetFeature` method that returns the named `Feature` for the
given `Point.`

```proto
rpc RecordRoute(stream Point) returns (RouteSummary) {}
```

#### RouteChat

Accepts a stream of `RouteNotes` sent while a route is being traversed, while
receiving other `RouteNotes` (e.g. from other users).

This is exactly the kind of usecase for *bidirectional streaming*.  A
bidirectional streaming RPC where both sides send a sequence of messages using a
read-write stream. The two streams operate independently, so clients and servers
can read and write in whatever order they like: for example, the server could
wait to receive all the client messages before writing its responses, or it
could alternately read a message then write a message, or some other combination
of reads and writes. The order of messages in each stream is preserved. You
specify this type of method by placing the stream keyword before both the
request and the response.

```proto
rpc RouteChat(stream RouteNote) returns (stream RouteNote) {}
```

### Compiling and Building Proto  
Create a `build.rs` file at the root of your crate. A build.rs script is a Rust program that Cargo executes before compiling your main project. Its purpose is to perform tasks like generating source code, linking to non-Rust libraries, or setting environment variables that influence the build process.

In this case, we will be putting the command to compile and build the `.proto` file in build.rs. We will use gRPC's tonic_protobuf_build crate to generate code from the `.proto` file.
```rust
fn main() {
    tonic_protobuf_build::CodeGen::new()
    .include("src/routeguide")
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



