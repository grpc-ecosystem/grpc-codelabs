# Getting Started with gRPC-Python

Get hands-on with gRPC for Python in this interactive codelab! 

Perfect for Python developers new to gRPC, those seeking a refresher, or anyone building distributed
systems. No prior gRPC experience needed!

**Build a complete gRPC service from scratch, learning:**

- Protocol Buffers (protobuf): Define service contracts & data.  
- gRPC Code Generation: Auto-generate Python code.  
- Client/Server Communication: Implement seamless interactions.

**You'll gain:**

- A working gRPC service in Python.  
- Hands-on experience with Protocol Buffers and code generation.  
- Skills to design, build, & test gRPC clients and servers.  
- A strong foundation in gRPC for real-world projects.

### How to use this directory

- [start_here](./start_here/) directory serves as a starting point for the codelab.  
- [completed](./completed/) directory showcases the finished code, giving you a peak of how the
  final implementation should look like.

## Prerequisites

### This codelab

```sh
cd ~/your-dev-dir
git clone https://github.com/grpc-ecosystem/grpc-codelabs.git
cd grpc-codelabs/
```

### Python3

For this codelab, we require python 3.9 or higher, but recommend python 3.11. System-specific
instructions can be found in Python documentation: [Python Setup and Usage](https://docs.python.org/3/using/index.html).

### Pip3

We recommend using the latest pip, see [Installation - pip](https://pip.pypa.io/en/stable/installation/).
In some OS distributions, `ensurepip` is not available out-of-box. On Debian/Ubuntu, you may need 
to run.

```sh
sudo apt-get install python3-pip
```

If necessary, upgrade your version of pip:

```sh
python3 -m ensurepip --upgrade
```

If your python installation is owned by the system, pip will be installed in the user directory. If
you may see a warning like this, ensure the pip directory is in `$PATH`:

```
WARNING: The scripts pip3 and pip3.9 are installed in '/Users/sergiitk/Library/Python/3.9/bin' which is not on PATH.
Consider adding this directory to PATH or, if you prefer to suppress this warning, use --no-warn-script-location.
```

### Venv

[venv](https://docs.python.org/3/library/venv.html) is a built-in tool to create python virtual
environments. However, some OS distributions choose to exclude it. You can check if it's available
on your system with

```sh
python3 -m venv --help
```

In debian/ubuntu, this also will advise you on what package to install. You may need to run
something like this:

```sh
sudo apt-get install python3-venv
```

Once `venv` is installed, create a virtual environment:

```sh
cd codelabs/grpc-python-getting-started
python3 -m venv .venv
```

#### Activate virtual environment

```sh
cd "$(git rev-parse --show-toplevel || echo .)" && cd codelabs/grpc-python-getting-started
source ./.venv/bin/activate
```

## Define proto

Your working directory will be `codelabs/grpc-python-getting-started/start_here`. Assuming you
followed `venv` activation section, you can cd into the start folder with:

```sh
cd start_here/
```

Our first step is to define the gRPC *service* and the method *request* and *response* types using
[protocol buffers](https://protobuf.dev/overview).  

Let’s start by defining the messages and service in `route_guide.proto`.

### Define proto messages

Our `.proto` file contains protocol buffer message type definitions for all the request and response
types used in our service methods.

Let’s define the `Point` message type:

```proto
message Point {
  int32 latitude = 1;
  int32 longitude = 2;
}
```

Let’s also define the `Feature` message type:

```proto
message Feature {
  // The name of the feature.
  string name = 1;

  // The point where the feature is detected.
  Point location = 2;
}
```

### Define RouteGuide service

To define a service, you specify a named `service` in your `.proto` file:

```proto
service RouteGuide {
  // Definition of the service goes here
}
```

### Define RPC Method

Then you define `rpc` methods inside your service definition, specifying their request and response
types.  In this section of the codelab, let’s define a Unary RPC method.

> Unary RPC method - A *simple RPC* where the client sends a request to the server using the stub
  and waits for a response to come back, just like a normal function call.

```proto
// Obtains the feature at a given position.
rpc GetFeature(Point) returns (Feature) {}
```

> [!TIP]
> For the complete .proto file, see
> [`completed/protos/route_guide.proto`](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-python-getting-started/completed/protos/route_guide.proto)

## Generating client and server code 

Next you need to generate the gRPC client and server interfaces from your .proto service definition.

First, install the grpcio-tools package:

```sh
pip install --require-virtualenv grpcio-tools
```

If you see `ERROR: Could not find an activated virtualenv (required)`, please 
[activate virtual environment](#activate-virtual-environment), then cd into `start_here`.

Use the following command to generate the Python code:

```sh
python -m grpc_tools.protoc --proto_path=./protos  \
 --python_out=. --pyi_out=. --grpc_python_out=. \
 ./protos/route_guide.proto
```

Note that as we’ve already provided a version of the generated code in the `completed` directory,
running this command regenerates the appropriate file rather than creating a new one. The generated
code files are called `route_guide_pb2.py` and `route_guide_pb2_grpc.py` and contain:

* classes for the messages defined in `route_guide.proto`  
* classes for the service defined in `route_guide.proto`  
  * `RouteGuideStub`, which can be used by clients to invoke RouteGuide RPCs  
  * `RouteGuideServicer`, which defines the interface for implementations of the RouteGuide service  
* a function for the service defined in `route_guide.proto`  
  * `add_RouteGuideServicer_to_server`, which adds a RouteGuideServicer to a `grpc.Server`.

> [!Note]
> The `2` in pb2 indicates that the generated code is following Protocol Buffers Python API version
> 2. Version 1 is obsolete. It has no relation to the Protocol Buffers Language version, which is
> the one indicated by `syntax = "proto3"` or `syntax = "proto2"` in a `.proto` file.


## Creating the server

First let’s look at how you create a `RouteGuide` server. Creating and running a `RouteGuide` server
breaks down into two work items:

* Implementing the servicer interface generated from our service definition with functions that
  perform the actual “work” of the service.  
* Running a gRPC server to listen for requests from clients and transmit responses.

You can find the initial `RouteGuide` server in [`start_here/route_guide_server.py`](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-python-getting-started/start_here/route_guide_server.py).

### Implementing RouteGuide

`route_guide_server.py` has a `RouteGuideServicer` class that subclasses the generated class
`route_guide_pb2_grpc.RouteGuideServicer`:

```py
# RouteGuideServicer provides an implementation
# of the methods of the RouteGuide service.
class RouteGuideServicer(route_guide_pb2_grpc.RouteGuideServicer):
```

`RouteGuideServicer` implements all the `RouteGuide` service methods.

Let us look into a simple RPC implementation in detail.  Method `GetFeature` gets a `Point` from the
client and returns the corresponding feature information from its database in `Feature`.

```py
def GetFeature(self, request, context):
    feature = get_feature(self.db, request)
    if feature is None:
        return route_guide_pb2.Feature(name="", location=request)
    else:
        return feature
```

The method is passed a `route_guide_pb2.Point` request for the RPC, and a `grpc.ServicerContext`
object that provides RPC-specific information such as timeout limits. It returns a
`route_guide_pb2.Feature` response.

> [!TIP]
> For the completed route guide server, see 
> [`completed/route_guide_server.py`](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-python-getting-started/completed/route_guide_server.py).

## Starting the server

Once you have implemented all the `RouteGuide` methods, the next step is to start up a gRPC server
so that clients can actually use your service:

```py
def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    route_guide_pb2_grpc.add_RouteGuideServicer_to_server(RouteGuideServicer(), server)
    listen_addr = "[::]:50051"
    server.add_insecure_port(listen_addr)
    print(f"Starting server on {listen_addr}")
    server.start()
    server.wait_for_termination()
```

The server `start()` method is non-blocking. A new thread will be instantiated to handle requests.
The thread calling `server.start()` will often not have any other work to do in the meantime. In
this case, you can call `server.wait_for_termination()` to cleanly block the calling thread until
the server terminates.

> [!TIP]
> For the completed route guide server, see 
> [`completed/route_guide_server.py`](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-python-getting-started/completed/route_guide_server.py).

## Creating the client

In this section, we’ll look at creating a client for our `RouteGuide` service. You can see the
initial client code in [`start_here/route_guide_client.py`](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-python-getting-started/start_here/route_guide_client.py).

### Creating a stub 

To call service methods, we first need to create a *stub*.

We instantiate the `RouteGuideStub` class of the `route_guide_pb2_grpc` module, generated from our
`.proto` inside of the `route_guide_client.py` file.

```py
channel = grpc.insecure_channel('localhost:50051')
stub = route_guide_pb2_grpc.RouteGuideStub(channel)
```

### Calling service methods 

For RPC methods that return a single response (“response-unary” methods), gRPC Python supports both
synchronous (blocking) and asynchronous (non-blocking) control flow semantics.

### Simple RPC 

First, let's define a `Point` to call the service with. This should be as simple as instantiating an
object from the `route_guide_pb2` package with some properties:

```py
point = route_guide_pb2.Point(latitude=412346009, longitude=-744026814)
```

A synchronous call to the simple RPC `GetFeature` is nearly as straightforward as calling a local
method. The RPC call waits for the server to respond, and will either return a response or raise an
exception. We can call the method and see the response like this:

```py
feature = stub.GetFeature(point)
print(feature)
```

You can inspect the fields of the Feature object and output the result of the request:

```py
if feature.name:
    print(f"Feature called '{feature.name}' at {format_point(feature.location)}")
else:
    print(f"Found no feature at at {format_point(feature.location)}")
```

> [!TIP]
> For the completed route guide client, see
> [`completed/route_guide_client.py`](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-python-getting-started/completed/route_guide_client.py).

## Try it out! 

Run the server:

```sh
python route_guide_server.py
```

From a different terminal, [activate virtual environment](#activate-virtual-environment), then run
the client:

```sh
python route_guide_client.py
```

## What’s next

* Learn how gRPC works in [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/) 
  and [Core concepts](https://grpc.io/docs/what-is-grpc/core-concepts/).  
* Explore the [Python API reference](https://grpc.github.io/grpc/python/).
