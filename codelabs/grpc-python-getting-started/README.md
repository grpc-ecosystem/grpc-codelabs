# Getting Started with gRPC-Python

Get hands-on with gRPC for Python in this interactive codelab! <!-- TODO(arvindbr8): Insert link once codelab is published. -->


Perfect for Python developers new to gRPC, those seeking a refresher, or anyone building distributed systems. No prior gRPC experience needed! 

#### Build a complete gRPC service from scratch, learning: 
- Protocol Buffers (protobuf): Define service contracts & data.
- gRPC Code Generation: Auto-generate Python code.
- Client/Server Communication: Implement seamless interactions.

#### You'll gain: 
- A working gRPC service in Python.
- Hands-on experience with Protocol Buffers and code generation.
- Skills to design, build, & test gRPC clients and servers.
- A strong foundation in gRPC for real-world projects.

## How to use this directory

- [start_here](start_here/) directory serves as a starting point for the
codelab.
- [completed](completed/) directory showcases the finished code, giving you a
peak of how the final implementation should look like.

---

# Before you begin

## **Prerequisites**

* Python 3.7 or higher  
* pip version 9.0.1 or higher

If necessary, upgrade your version of pip:

```
python3 -m pip install --upgrade pip
```

If you cannot upgrade pip due to a system-owned installation, you can run the example in a virtualenv:

```
python3 -m pip install virtualenv
python3 -m venv venv --upgrade-deps
source venv/bin/activate
```

## **What you’ll learn**

* Get hands-on with gRPC for Python in this interactive codelab\! Perfect for Python developers new to gRPC, those seeking a refresher, or anyone building distributed systems. No prior gRPC experience needed\!   
* Build a complete gRPC service from scratch, learning:   
  * Protocol Buffers (protobuf): Define service contracts & data.   
  * gRPC Code Generation: Auto-generate Python code.   
  * Client/Server Communication: Implement seamless interactions.   
  * Testing & Debugging: Ensure reliability & correctness.   
* You'll gain:   
  * A working gRPC service in Python.   
  * Hands-on experience with Protocol Buffers and code generation.   
  * Skills to design, build, & test gRPC clients and servers.   
  * A strong foundation in gRPC for real-world projects.

## **What you’ll need**

* A computer with internet connection

---

# Define proto

Duration: 5:00

Our first step is to define the gRPC *service* and the method *request* and *response* types using [protocol buffers](https://protobuf.dev/overview). 

| Hint: For the complete .proto file, see [protos/route\_guide.proto.](https://github.com/grpc-ecosystem/grpc-codelabs/blob/aa70365c774eab3f3a13a086599a596a4612f3ff/codelabs/Getting\_Started\_with\_gRPC\_Python/completed/protos/route\_guide.proto) |
| :---- |

Let’s create a `route_guide.proto` file.

## **Define proto Message**

Our `.proto` file contains protocol buffer message type definitions for all the request and response types used in our service methods.

Let’s define the `Point` message type:

```
// Points are represented as latitude-longitude pairs in the E7 representation
// (degrees multiplied by 10**7 and rounded to the nearest integer).
// Latitudes should be in the range +/- 90 degrees and longitude should be in
// the range +/- 180 degrees (inclusive).
message Point {
  int32 latitude = 1;
  int32 longitude = 2;
}
```

## 

Let’s also define the `Feature` message type:

```
// A feature names something at a given point.
//
// If a feature could not be named, the name is empty.
message Feature {
  // The name of the feature.
  string name = 1;

  // The point where the feature is detected.
  Point location = 2;
}
```

## **Define RouteGuide service**

To define a service, you specify a named `service` in your `.proto` file:

```
service RouteGuide {
  // Definition of the service goes here
}
```

## **Define RPC Method**

Then you define `rpc` methods inside your service definition, specifying their request and response types.  In this section of the codelab, let’s define 

* Unary RPC method \- A *simple RPC* where the client sends a request to the server using the stub and waits for a response to come back, just like a normal function call.

```
// Obtains the feature at a given position.
rpc GetFeature(Point) returns (Feature) {}
```

---

# Generating client and server code 

Next you need to generate the gRPC client and server interfaces from your .proto service definition.

First, install the grpcio-tools package:

```
pip install grpcio-tools
```

Use the following command to generate the Python code:

```
python -m grpc_tools.protoc -I./protos --python_out=. \
 --pyi_out=. --grpc_python_out=. ./protos/route_guide.proto
```

Note that as we’ve already provided a version of the generated code in the `completed` directory, running this command regenerates the appropriate file rather than creating a new one. The generated code files are called `route_guide_pb2.py` and `route_guide_pb2_grpc.py` and contain:

* classes for the messages defined in `route_guide.proto`  
* classes for the service defined in `route_guide.proto`  
  * `RouteGuideStub`, which can be used by clients to invoke RouteGuide RPCs  
  * `RouteGuideServicer`, which defines the interface for implementations of the RouteGuide service  
* a function for the service defined in `route_guide.proto`  
  * `add_RouteGuideServicer_to_server`, which adds a RouteGuideServicer to a `grpc.Server`.

| Note: The `2` in pb2 indicates that the generated code is following Protocol Buffers Python API version 2\. Version 1 is obsolete. It has no relation to the Protocol Buffers Language version, which is the one indicated by `syntax = "proto3"` or `syntax = "proto2"` in a `.proto` file. |
| :---- |

## **Generating gRPC interfaces with custom package path** 

## 

To generate gRPC client interfaces with a custom package path, you can use the \-I parameter along with the grpc\_tools.protoc command. This approach allows you to specify a custom package name for the generated files.

Here’s an example command to generate the gRPC client interfaces with a custom package path:

```
$ python -m grpc_tools.protoc \
  -Igrpc/example/custom/path=./protos \
  --python_out=. --grpc_python_out=. \
  ./protos/route_guide.proto
```

The generated files will be placed in the `./grpc/example/custom/path/` directory:

* `./grpc/example/custom/path/route_guide_pb2.py`  
* `./grpc/example/custom/path/route_guide_pb2_grpc.py`

With this setup, the generated `route_guide_pb2_grpc.py` file will correctly import the protobuf definitions using the custom package structure, as shown below:

```py
import grpc.example.custom.path.route_guide_pb2 as route_guide_pb2
```

By following this approach, you can ensure that the files will call each other correctly with respect to the specified package path. This method allows you to maintain a custom package structure for your gRPC client interfaces.

---

# Creating the server

Duration: 5:00

First let’s look at how you create a `RouteGuide` server. Creating and running a `RouteGuide` server breaks down into two work items:

* Implementing the servicer interface generated from our service definition with functions that perform the actual “work” of the service.  
* Running a gRPC server to listen for requests from clients and transmit responses.

You can find the example `RouteGuide` server in [`examples/python/route_guide/route_guide_server.py`](https://github.com/grpc/grpc/blob/v1.64.0/examples/python/route\_guide/route\_guide\_server.py).

## **Implementing RouteGuide**

`route_guide_server.py` has a `RouteGuideServicer` class that subclasses the generated class `route_guide_pb2_grpc.RouteGuideServicer`:

```py
# RouteGuideServicer provides an implementation of the methods of the RouteGuide service.
class RouteGuideServicer(route_guide_pb2_grpc.RouteGuideServicer):
```

`RouteGuideServicer` implements all the `RouteGuide` service methods.

Let us look into a simple RPC implementation in detail

### **Unary RPC**

Let’s look at the simplest type first, `GetFeature`, which just gets a `Point` from the client and returns the corresponding feature information from its database in `Feature`.

```py
def GetFeature(self, request, context):
    feature = get_feature(self.db, request)
    if feature is None:
        return route_guide_pb2.Feature(name="", location=request)
    else:
        return feature
```

The method is passed a `route_guide_pb2.Point` request for the RPC, and a `grpc.ServicerContext` object that provides RPC-specific information such as timeout limits. It returns a `route_guide_pb2.Feature` response.

---

# Starting the server

Duration: 5:00

Once you have implemented all the `RouteGuide` methods, the next step is to start up a gRPC server so that clients can actually use your service:

```py
def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    route_guide_pb2_grpc.add_RouteGuideServicer_to_server(RouteGuideServicer(), server)
    server.add_insecure_port("[::]:50051")
    server.start()
    server.wait_for_termination()
```

The server `start()` method is non-blocking. A new thread will be instantiated to handle requests. The thread calling `server.start()` will often not have any other work to do in the meantime. In this case, you can call `server.wait_for_termination()` to cleanly block the calling thread until the server terminates.

---

# Creating the client

Duration: 5:00

In this section, we’ll look at creating a client for our `RouteGuide` service. You can see our complete example client code in [`examples/python/route_guide/route_guide_client.py`](https://github.com/grpc/grpc/blob/v1.64.0/examples/python/route\_guide/route\_guide\_client.py)

## **Creating a stub** 

To call service methods, we first need to create a *stub*.

We instantiate the `RouteGuideStub` class of the `route_guide_pb2_grpc` module, generated from our `.proto` inside of the `route_guide_client.py` file.

```py
channel = grpc.insecure_channel('localhost:50051')
stub = route_guide_pb2_grpc.RouteGuideStub(channel)
```

## 

## **Calling service methods** 

For RPC methods that return a single response (“response-unary” methods), gRPC Python supports both synchronous (blocking) and asynchronous (non-blocking) control flow semantics.

### **Simple RPC** 

First, let's define a `Point` to call the service with. This should be as simple as instantiating an object from the `route_guide_pb2` package with some properties:

```py
point = route_guide_pb2.Point(latitude=409146138, longitude=-746188906)
```

A synchronous call to the simple RPC `GetFeature` is nearly as straightforward as calling a local method. The RPC call waits for the server to respond, and will either return a response or raise an exception. We can call the method and see the response like this:

```py
feature = stub.GetFeature(point)
print(feature)
```

---

# 

# Try it out\! 

Duration: 2:00

Run the server:

```
python route_guide_server.py
```

From a different terminal, run the client:

```
python route_guide_server.py
```

---

# What’s next

* Learn how gRPC works in [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/) and [Core concepts](https://grpc.io/docs/what-is-grpc/core-concepts/).  
* Explore the [API reference](https://grpc.io/docs/languages/go/api).

---
