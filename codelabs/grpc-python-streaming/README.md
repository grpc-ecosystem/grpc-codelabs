# Getting Started with gRPC-Python (Streaming)

Get hands-on with gRPC for Python in this interactive codelab! 

Perfect for Python developers new to gRPC, those seeking a refresher, or anyone building distributed systems. No prior gRPC experience needed!

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

- [start_here](http://start_here/) directory serves as a starting point for the codelab.  
- [completed](http://completed/) directory showcases the finished code, giving you a peak of how the final implementation should look like.

## Prerequisites

If you are here after completing Getting Started with gRPC-Python codelab, you can skip this step.

### This codelab

```
cd ~/your-dev-dir
git clone https://github.com/grpc-ecosystem/grpc-codelabs.git
cd grpc-codelabs/
```

### Python3

For this codelab, we require python 3.9 or higher, but recommend python 3.11.  
System-specific instructions can be found in Python documentation: [Python Setup and Usage](https://docs.python.org/3/using/index.html).

### Pip3

We recommend using the latest pip, see [Installation - pip](https://pip.pypa.io/en/stable/installation/).  
In some OS distributions, `ensurepip` is not available out-of-box. On Debian/Ubuntu, you may need to run.

```
sudo apt-get install python3-pip
```

 If necessary, upgrade your version of pip:

```
python3 -m ensurepip --upgrade
```

If your python installation is owned by the system, pip will be installed in the user directory. If you may see a warning like this, ensure the pip directory is in PATH:

```
WARNING: The scripts pip3 and pip3.9 are installed in '/Users/sergiitk/Library/Python/3.9/bin' which is not on PATH.
Consider adding this directory to PATH or, if you prefer to suppress this warning, use --no-warn-script-location.
```

### Venv

[venv](https://docs.python.org/3/library/venv.html) is a built-in tool to create python virtual environments. However, some OS distributions choose to exclude it. You can check if it's available on your system with

```
python3 -m venv --help
```

In debian/ubuntu, this also will advise you on what package to install. You may need to run something like this:

```
sudo apt-get install python3-venv
```

Once `venv` is installed, create a virtual environment:

```
cd codelabs/grpc-python-streaming
python3 -m venv .venv
```

#### Activate virtual environment

```
cd "$(git rev-parse --show-toplevel || echo .)" && cd codelabs/grpc-python-streaming
source ./.venv/bin/activate
```

## Define proto

Duration: 5:00

Your working directory will be `codelabs/grpc-python-streaming/start_here`. Assuming you followed `venv` activation section, you can cd into the start folder with:

```
cd start_here/
```

Our first step is to define the gRPC *service* and the method *request* and *response* types using [protocol buffers](https://protobuf.dev/overview). 

Let’s create a `route_guide.proto` file.

### Define proto Message

Our `.proto` file contains protocol buffer message type definitions for all the request and response types used in our service methods - let’s define the Point message type:

```
message Point {
  int32 latitude = 1;
  int32 longitude = 2;
}
```

Let’s also define the `Feature` message type:

```
message Feature {
  // The name of the feature.
  string name = 1;

  // The point where the feature is detected.
  Point location = 2;
}
```

And `Rectangle` message type:

```
message Rectangle {
  // One corner of the rectangle.
  Point lo = 1;

  // The other corner of the rectangle.
  Point hi = 2;
}
```

Also a `RouteNote` message which represents a message sent while at a given point.

```
message RouteNote {
  // The location from which the message is sent.
  Point location = 1;

  // The message to be sent.
  string message = 2;
}
```

We would also require a `RouteSummary` message.  This message is received in response to a `RecordRoute` rpc which is explained in the next section.  It contains the number of individual points received, the number of detected features, and the total distance covered as the cumulative sum of the distance between each point.

```
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

### Define Route Guide service

To define a service, you specify a named `service` in your `.proto` file:

```
service RouteGuide {
  // Definition of the service goes here
}
```

### Define RPC Methods

Then you define `rpc` methods inside your service definition, specifying their request and response types.  In this section of the codelab, let’s define:

#### ListFeatures

Obtains the Features available within the given Rectangle. Results are streamed rather than returned at once (e.g. in a response message with a repeated field), as the rectangle may cover a large area and contain a huge number of features.

An appropriate type for this RPC is *server-side* streaming RPC. A server-side streaming RPC where the client sends a request to the server and gets a stream to read a sequence of messages back. The client reads from the returned stream until there are no more messages. As you can see in our example, you specify a server-side streaming method by placing the stream keyword before the response type.

```
rpc ListFeatures(Rectangle) returns (stream Feature) {}
```

#### RecordRoute

Accepts a stream of Points on a route being traversed, returning a `RouteSummary` when traversal is completed.

A *client-side streaming* RPC seems appropriate in this case.  A client-side streaming RPC where the client writes a sequence of messages and sends them to the server, again using a provided stream. Once the client has finished writing the messages, it waits for the server to read them all and return its response. You specify a client-side streaming method by placing the stream keyword before the request type. `GetFeature` method that returns the named `Feature` for the given `Point.`

```
rpc RecordRoute(stream Point) returns (RouteSummary) {}
```

#### RouteChat

Accepts a stream of `RouteNotes` sent while a route is being traversed, while receiving other `RouteNotes` (e.g. from other users).

This is exactly the kind of use case for *bidirectional streaming*. A bidirectional streaming RPC where both sides send a sequence of messages using a read-write stream. The two streams operate independently, so clients and servers can read and write in whatever order they like: for example, the server could wait to receive all the client messages before writing its responses, or it could alternately read a message then write a message, or some other combination of reads and writes. The order of messages in each stream is preserved. You specify this type of method by placing the stream keyword before both the request and the response.

```
rpc RouteChat(stream RouteNote) returns (stream RouteNote) {}
```

| Hint: For the complete .proto file, see [completed/protos/route_guide.proto](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-python-streaming/completed/protos/route_guide.proto) |
| :---- |

## Generating client and server code 

Next you need to generate the gRPC client and server interfaces from your `.proto` service definition.

First, install the grpcio-tools package:

```
pip install --require-virtualenv grpcio-tools
```

If you see `ERROR: Could not find an activated virtualenv (required)`, please follow the section [activate virtual environment](https://docs.google.com/document/d/12oMqscjN_UA6GSpdr09EUJ1vPYGntXbZiZQI96lyvT4/edit?resourcekey=0-GbVLembJB-Bz4x2hsgpioQ\&tab=t.0\#heading=h.dks8uqsprozy), then cd into `start_here`.

Use the following command to generate the Python code:

```
python -m grpc_tools.protoc --proto_path=./protos  \
 --python_out=. --pyi_out=. --grpc_python_out=. \
 ./protos/route_guide.proto
```

Note that as we’ve already provided a version of the generated code in the `completed` directory, running this command regenerates the appropriate file rather than creating a new one. The generated code files are called `route_guide_pb2.py` and `route_guide_pb2_grpc.py` and contain:

* classes for the messages defined in `route_guide.proto`  
* classes for the service defined in `route_guide.proto`  
  * `RouteGuideStub`, which can be used by clients to invoke RouteGuide RPCs  
  * `RouteGuideServicer`, which defines the interface for implementations of the RouteGuide service  
* a function for the service defined in `route_guide.proto`  
  * `add_RouteGuideServicer_to_server`, which adds a RouteGuideServicer to a `grpc.Server`.

| Note: The 2 in pb2 indicates that the generated code is following Protocol Buffers Python API version 2\. Version 1 is obsolete. It has no relation to the Protocol Buffers Language version, which is the one indicated by syntax \= "proto3" or syntax \= "proto2" in a .proto file. |
| :---- |

## Creating the server

Duration: 5:00

First let’s look at how you create a RouteGuide server. Creating and running a RouteGuide server breaks down into two work items:

* Implementing the servicer interface generated from our service definition with functions that perform the actual “work” of the service.  
* Running a gRPC server to listen for requests from clients and transmit responses.

You can find the initial `RouteGuide` server in [`start_here/route_guide_server.py`](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-python-streaming/start_here/route_guide_server.py).

### Implementing RouteGuide

`route_guide_server.py` has a `RouteGuideServicer` class that subclasses the generated class `route_guide_pb2_grpc.RouteGuideServicer`:

```py
# RouteGuideServicer provides an implementation of the methods of the RouteGuide service.
class RouteGuideServicer(route_guide_pb2_grpc.RouteGuideServicer):
```

`RouteGuideServicer` implements all the `RouteGuide` service methods.

### Server-side streaming RPC 

`ListFeatures` is a response-streaming RPC that sends multiple `Feature`s to the client.

```py
def ListFeatures(self, request, context):
    left = min(request.lo.longitude, request.hi.longitude)
    right = max(request.lo.longitude, request.hi.longitude)
    top = max(request.lo.latitude, request.hi.latitude)
    bottom = min(request.lo.latitude, request.hi.latitude)
    for feature in self.db:
        if (
            feature.location.longitude >= left
            and feature.location.longitude <= right
            and feature.location.latitude >= bottom
            and feature.location.latitude <= top
        ):
            yield feature
```

Here the request message is a `route_guide_pb2.Rectangle` within which the client wants to find `Feature`s. Instead of returning a single response the method yields zero or more responses.

### Client-side streaming RPC 

The request-streaming method `RecordRoute` uses an [iterator](https://docs.python.org/3/library/stdtypes.html\#iterator-types) of request values and returns a single response value.

```py
def RecordRoute(self, request_iterator, context):
    point_count = 0
    feature_count = 0
    distance = 0.0
    prev_point = None

    start_time = time.time()
    for point in request_iterator:
        point_count += 1
        if get_feature(self.db, point):
            feature_count += 1
        if prev_point:
            distance += get_distance(prev_point, point)
        prev_point = point

    elapsed_time = time.time() - start_time
    return route_guide_pb2.RouteSummary(
        point_count=point_count,
        feature_count=feature_count,
        distance=int(distance),
        elapsed_time=int(elapsed_time),
    )
```

### Bidirectional streaming RPC 

Finally, let’s look at our bidirectional streaming RPC `RouteChat()`.

```py
def RouteChat(self, request_iterator, context):
    prev_notes = []
    for new_note in request_iterator:
        for prev_note in prev_notes:
            if prev_note.location == new_note.location:
                yield prev_note
        prev_notes.append(new_note)
```

This method’s semantics are a combination of those of the request-streaming method and the response-streaming method. It is passed an iterator of request values and is itself an iterator of response values.

| Hint: For the completed route guide server, see [completed/route_guide_server.py](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-python-streaming/completed/route_guide_server.py).  |
| :---- |

## Starting the server

Duration: 5:00

Once you have implemented all the `RouteGuide` methods, the next step is to start up a gRPC server so that clients can actually use your service:

```py
def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    route_guide_pb2_grpc.add_RouteGuideServicer_to_server(RouteGuideServicer(), server)
    listen_addr = "localhost:50051"
    server.add_insecure_port(listen_addr)
    print(f"Starting server on {listen_addr}")
    server.start()
    server.wait_for_termination()
```

The server `start()` method is non-blocking. A new thread will be instantiated to handle requests. The thread calling `server.start()` will often not have any other work to do in the meantime. In this case, you can call `server.wait_for_termination()` to cleanly block the calling thread until the server terminates.

| Hint: For the completed route guide server, see [completed/route_guide_server.py](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-python-streaming/completed/route_guide_server.py).  |
| :---- |

## Creating the client

Duration: 5:00

In this section, we’ll look at creating a client for our RouteGuide service. You can see the initial client code in [`start_here/route_guide_client.py`](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-python-streaming/start_here/route_guide_client.py).

### Creating a stub 

To call service methods, we first need to create a *stub*.

We instantiate the `RouteGuideStub` class of the `route_guide_pb2_grpc` module, generated from our `.proto.` In `run()` method:

```py
with grpc.insecure_channel("localhost:50051") as channel:
    stub = route_guide_pb2_grpc.RouteGuideStub(channel)
```

Note that here `channel` is used as a context manager, and will be automatically closed once the interpreter leaves the `with` block.

### Calling service methods 

For RPC methods that return a single response ("response-unary" methods), gRPC Python supports both synchronous (blocking) and asynchronous (non-blocking) control flow semantics. For response-streaming RPC methods, calls immediately return an iterator of response values. Calls to that iterator’s `next()` method block until the response to be yielded from the iterator becomes available.

### Server-side streaming RPC 

Calling the response-streaming `ListFeatures` is similar to working with sequence types:

```py
def guide_list_features(stub):
    _lo = route_guide_pb2.Point(latitude=400000000, longitude=-750000000)
    _hi = route_guide_pb2.Point(latitude=420000000, longitude=-730000000)
    rectangle = route_guide_pb2.Rectangle(
        lo=_lo,
        hi=_hi,
    )
    print("Looking for features between 40, -75 and 42, -73")

    features = stub.ListFeatures(rectangle)

    for feature in features:
        print(f"Feature called '{feature.name}' at {format_point(feature.location)}")
```

### Client-side streaming RPC

Calling the request-streaming `RecordRoute` is similar to passing an iterator to a local method. Like the simple RPC above that also returns a single response, it can be called synchronously:

```py
def guide_record_route(stub):
    feature_list = route_guide_resources.read_route_guide_database()
    route_iterator = generate_route(feature_list)

    route_summary = stub.RecordRoute(route_iterator)
    print(f"Finished trip with {route_summary.point_count} points")
    print(f"Passed {route_summary.feature_count} features")
    print(f"Traveled {route_summary.distance} meters")
    print(f"It took {route_summary.elapsed_time} seconds")
```

### Bidirectional streaming RPC 

Calling the bidirectionally-streaming RouteChat has (as is the case on the service-side) a combination of the request-streaming and response-streaming semantics:

```py
def make_route_note(message, latitude, longitude):
    return route_guide_pb2.RouteNote(
        message=message,
        location=route_guide_pb2.Point(latitude=latitude, longitude=longitude),
    )
```

```py
def guide_route_chat(stub):
    responses = stub.RouteChat(generate_messages())
    for response in responses:
        print(
            f"Received message {response.message} at {format_point(response.location)}"
        )
```

### Call the helper methods

In run, execute the methods we just created, and pass them the `stub`.

```py
print("-------------- ListFeatures --------------")
guide_list_features(stub)
print("-------------- RecordRoute --------------")
guide_record_route(stub)
print("-------------- RouteChat --------------")
guide_route_chat(stub)
```

## Try it out! 

Duration: 2:00

Run the server:

```
python route_guide_server.py
```

From a different terminal, [activate virtual environment](https://docs.google.com/document/d/12oMqscjN_UA6GSpdr09EUJ1vPYGntXbZiZQI96lyvT4/edit?resourcekey=0-GbVLembJB-Bz4x2hsgpioQ\&tab=t.0\#heading=h.dks8uqsprozy), then run the client:

```
python route_guide_client.py
```

## What’s next

* Learn how gRPC works in [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/) and [Core concepts](https://grpc.io/docs/what-is-grpc/core-concepts/).  
* Explore the [Python API reference](https://grpc.github.io/grpc/python/).
