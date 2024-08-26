# Getting Started with gRPC-Go (Streaming)

Get hands-on with gRPC for Go in this interactive codelab! <!-- TODO(arvindbr8): Insert link once codelab is published. -->

Perfect for Go developers new to gRPC, those seeking a refresher, or anyone
building distributed systems. No prior gRPC experience needed!

## How to use this directory

- [start_here](start_here/) directory serves as a starting point for the
codelab.
- [completed](completed/) directory showcases the finished code, giving you a
peak of how the final implementation should look like.

## Before you begin

### What you’ll learn

* Get hands-on with gRPC for Go in this interactive codelab\! Perfect for Go
  developers new to gRPC, those seeking a refresher, or anyone building
  distributed systems. No prior gRPC experience needed\!
* Build a complete gRPC service from scratch, learning:
  * Protocol Buffers (protobuf): Define service contracts & data.
  * gRPC Code Generation: Auto-generate Go code.
  * Client/Server Communication: Implement seamless interactions.
  * Testing & Debugging: Ensure reliability & correctness.
* You'll gain:
  * A working gRPC service in Go.
  * Hands-on experience with Protocol Buffers and code generation.
  * Skills to design, build, & test gRPC clients and servers.
  * A strong foundation in gRPC for real-world projects.

### What you’ll need

* A computer with internet connection

### What you'll build

Our example is a simple route mapping application that lets clients get
information about features on their route, create a summary of their route, and
exchange route information such as traffic updates with the server and other
clients.

With gRPC we can define our service once in a `.proto` file and generate clients
and servers in any of gRPC’s supported languages, which in turn can be run in
environments ranging from servers inside a large data center to your own tablet
— all the complexity of communication between different languages and
environments is handled for you by gRPC. We also get all the advantages of
working with protocol buffers, including efficient serialization, a simple IDL,
and easy interface updating.

### Prerequisites

* [**Go**](https://golang.org/), any one of the **two latest major** [releases of Go](https://golang.org/doc/devel/release.html).
    * For installation instructions, see Go’s [Getting Started](https://golang.org/doc/install) guide.
* [**Protocol buffer**](https://developers.google.com/protocol-buffers) **compiler**, `protoc`, [version 3](https://protobuf.dev/programming-guides/proto3).
    * For installation instructions, see [Protocol Buffer Compiler Installation](https://grpc.io/docs/protoc-installation/).
* **Go plugins** for the protocol compiler:
    * Install the protocol compiler plugins for Go using the following commands.

```console
# This command will install the plugin that generates code for the messages
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest

# This command installs the plugin that generates code for the services and methods
go install google.golang.org/protobuf/cmd/protoc-gen-go-grpc@latest
```

* Update your PATH so that the protoc compiler can find the plugins:

```sh
export PATH="$PATH:$(go env GOPATH)/bin"
```

* Use [this as a starting point](https://download-directory.github.io/?url=https%3A%2F%2Fgithub.com%2Fgrpc-ecosystem%2Fgrpc-codelabs%2Ftree%2Fmain%2Fcodelabs%2FGetting\_Started\_with\_gRPC\_Go) for this codelab.

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

> [!TIP]
>  For the complete .proto file, see [routeguide/route_guide.proto](/completed/routeguide/route_guide.proto).

## Generating client and server code

Next we need to generate the gRPC client and server interfaces from our `.proto`
service definition. We do this using the protocol buffer compiler `protoc` with
a special gRPC Go plugin.

In the same directory where `route_guide.proto` is located, run the following command:

```sh
protoc --go_out=. --go_opt=paths=source_relative \
    --go-grpc_out=. --go-grpc_opt=paths=source_relative \
    route_guide.proto
```

Running this command generates the following files in the [routeguide](start_here/routeguide) directory:

* `route_guide.pb.go`, which contains all the protocol buffer code to populate,
  serialize, and retrieve request and response message types.
* `route_guide_grpc.pb.go`, which contains the following:
  * An interface type (or *stub*) for clients to call with the methods defined
    in the `RouteGuide` service.
  * An interface type for servers to implement, also with the methods defined in
    the `RouteGuide` service.

## Creating the server

First let’s look at how we create a `RouteGuide` server. There are two parts to
making our `RouteGuide` service do its job:

* Implementing the service interface generated from our service definition:
  doing the actual “work” of our service.
* Running a gRPC server to listen for requests from clients and dispatch them to
  the right service implementation.

> [!TIP]
>  For the complete server implementation, see [server.go](completed/server/server.go)

Let’s implement RouteGuide in `server/server.go`

### Implementing RouteGuide

We need to implement the generated `RouteGuideService` interface. This is how
the implementation would look

> [!Note]
>  The starter code already has helper function which will load features into the routeGuideServer's savedFeatures field.

```go
type routeGuideServer struct {
        ...
}
...
func (s *routeGuideServer) ListFeatures(rect *pb.Rectangle, stream pb.RouteGuide_ListFeaturesServer) error {
        ...
}
...

func (s *routeGuideServer) RecordRoute(stream pb.RouteGuide_RecordRouteServer) error {
        ...
}
...

func (s *routeGuideServer) RouteChat(stream pb.RouteGuide_RouteChatServer) error {
        ...
}
```

Let us look into the RPC implementation in detail

#### Server-side streaming RPC 

Now let’s look at one of our streaming RPCs. `ListFeatures` is a server-side
streaming RPC, so we need to send back multiple `Feature` s to our client

```go
func (s *routeGuideServer) ListFeatures(rect *pb.Rectangle, stream pb.RouteGuide_ListFeaturesServer) error {
  for _, feature := range s.savedFeatures {
    if inRange(feature.Location, rect) {
      if err := stream.Send(feature); err != nil {
        return err
      }
    }
  }
  return nil
}
```

As you can see, instead of getting simple request and response objects in our
method parameters, this time we get a request object (the `Rectangle` in which
our client wants to find `Features`) and a special
`RouteGuide_ListFeaturesServer` object to write our responses. In the method, we
populate as many `Feature` objects as we need to return, writing them to the
`RouteGuide_ListFeaturesServer` using its `Send()` method. Finally, as in our
simple RPC, we return a nil error to tell gRPC that we’ve finished writing
responses. Should any error happen in this call, we return a non-nil error; the
gRPC layer will translate it into an appropriate RPC status to be sent on the
wire.

#### Client-side streaming RPC 

Now let’s look at something a little more complicated: the client-side streaming
method `RecordRoute`, where we get a stream of `Points` from the client and
return a single `RouteSummary` with information about their trip. As you can
see, this time the method doesn’t have a request parameter at all. Instead, it
gets a `RouteGuide_RecordRouteServer` stream, which the server can use to both
read and write messages - it can receive client messages using its `Recv()`
method and return its single response using its `SendAndClose()` method.

```go
func (s *routeGuideServer) RecordRoute(stream pb.RouteGuide_RecordRouteServer) error {
  var pointCount, featureCount, distance int32
  var lastPoint *pb.Point
  startTime := time.Now()
  for {
    point, err := stream.Recv()
    if err == io.EOF {
      endTime := time.Now()
      return stream.SendAndClose(&pb.RouteSummary{
        PointCount:   pointCount,
        FeatureCount: featureCount,
        Distance:     distance,
        ElapsedTime:  int32(endTime.Sub(startTime).Seconds()),
      })
    }
    if err != nil {
      return err
    }
    pointCount++
    for _, feature := range s.savedFeatures {
      if proto.Equal(feature.Location, point) {
        featureCount++
      }
    }
    if lastPoint != nil {
      distance += calcDistance(lastPoint, point)
    }
    lastPoint = point
  }
}
```

In the method body we use the `RouteGuide_RecordRouteServer`’s `Recv()` method
to repeatedly read in our client’s requests to a request object (in this case a
`Point`) until there are no more messages: the server needs to check the error
returned from `Recv()` after each call. If this is nil, the stream is still good
and it can continue reading; if it’s `io.EOF` the message stream has ended and
the server can return its `RouteSummary`. If it has any other value, we return
the error “as is” so that it’ll be translated to an RPC status by the gRPC
layer.

#### Bidirectional streaming RPC 

Finally, let’s look at our bidirectional streaming RPC `RouteChat()`.

```go
func (s *routeGuideServer) RouteChat(stream pb.RouteGuide_RouteChatServer) error {
  for {
    in, err := stream.Recv()
    if err == io.EOF {
      return nil
    }
    if err != nil {
      return err
    }
    key := serialize(in.Location)
                ... // look for notes to be sent to client
    for _, note := range s.routeNotes[key] {
      if err := stream.Send(note); err != nil {
        return err
      }
    }
  }
}
```

This time we get a `RouteGuide_RouteChatServer` stream that, as in our
client-side streaming example, can be used to read and write messages. However,
this time we return values via our method’s stream while the client is still
writing messages to their message stream. The syntax for reading and writing
here is very similar to our client-streaming method, except the server uses the
stream’s Send() method rather than `SendAndClose()` because it’s writing
multiple responses. Although each side will always get the other’s messages in
the order they were written, both the client and server can read and write in
any order — the streams operate completely independently.

## Starting the server

Once we’ve implemented all our methods, we also need to start up a gRPC server
so that clients can actually use our service. The following snippet shows how we
do this for our `RouteGuide` service:

> [!NOTE]
>  port can be configured by passing in `port` flag. Defaults to `50051`

```go
lis, err := net.Listen("tcp", fmt.Sprintf("localhost:%d", port))
if err != nil {
  log.Fatalf("failed to listen: %v", err)
}
var opts []grpc.ServerOption
grpcServer := grpc.NewServer(opts...)

s := &routeGuideServer{}
s.loadFeatures()
pb.RegisterRouteGuideServer(grpcServer, newServer())
grpcServer.Serve(lis)
```

To build and start a server, we:

1. Specify the port we want to use to listen for client requests using:
   `lis, err := net.Listen(...)`
2. Create an instance of the gRPC server using `grpc.NewServer(...)`.
3. Use `s.loadFeatures()` to load features into `s.savedFeatures`
4. Register our service implementation with the gRPC server.
5. Call `Serve()` on the server with our port details to do a blocking wait
   until the process is killed or `Stop()` is called.

## Creating the client

In this section, we’ll look at creating a Go client for our RouteGuide service.

> [!TIP]
>  For the complete server implementation, see [client.go](completed/client/client.go)

### Creating a stub

To call service methods, we first need to create a gRPC *channel* to communicate
with the server. We create this by passing the server address and port number to
`grpc.NewClient()` as follows:

> [!NOTE]
>  serverAddr can be configured by passing in `addr` flag. Defaults to `localhost:50051`

```go
conn, err := grpc.NewClient("dns:///"+*serverAddr, grpc.WithTransportCredentials(insecure.NewCredentials()))
if err != nil {
  log.Fatalf("fail to dial: %v", err)
}
defer conn.Close()
```

You can use `DialOptions` to set the auth credentials (for example, TLS, GCE
credentials, or JWT credentials) in `grpc.NewClient` when a service requires
them. The `RouteGuide` service doesn’t require any credentials.

Once the gRPC *channel* is set up, we need a client *stub* to perform RPCs by
making Go function calls. We get it using the `NewRouteGuideClient` method
provided by the pb package generated from the example `.proto` file.

```go
client := pb.NewRouteGuideClient(conn)
```

### Calling service methods

Now let’s look at how we call our service methods. Note that in gRPC-Go, RPCs
operate in a blocking/synchronous mode, which means that the RPC call waits for
the server to respond, and will either return a response or an error.

#### Server-side streaming RPC 

Here’s where we call the server-side streaming method `ListFeatures`, which
returns a stream of geographical `Feature`s.

```go
rect := &pb.Rectangle{ ... }  // initialize a pb.Rectangle
stream, err := client.ListFeatures(context.Background(), rect)
if err != nil {
  ...
}
for {
    feature, err := stream.Recv()
    if err == io.EOF {
        break
    }
    if err != nil {
        log.Fatalf("%v.ListFeatures(_) = _, %v", client, err)
    }
    log.Println(feature)
}
```

As in the simple RPC, we pass the method a context and a request. However,
instead of getting a response object back, we get back an instance of
`RouteGuide_ListFeaturesClient`. The client can use the
`RouteGuide_ListFeaturesClient` stream to read the server’s responses. We use
the `RouteGuide_ListFeaturesClient`’s `Recv()` method to repeatedly read in the
server’s responses to a response protocol buffer object (in this case a
`Feature`) until there are no more messages: the client needs to check the error
err returned from `Recv()` after each call. If nil, the stream is still good and
it can continue reading; if it’s `io.EOF` then the message stream has ended;
otherwise there must be an RPC error, which is passed over through `err`.

#### Client-side streaming RPC 

The client-side streaming method `RecordRoute` is similar to the server-side
method, except that we only pass the method a context and get a
`RouteGuide_RecordRouteClient` stream back, which we can use to both *write* and
*read* messages.

```go
// Create a random number of random points
r := rand.New(rand.NewSource(time.Now().UnixNano()))
pointCount := int(r.Int31n(100)) + 2 // Traverse at least two points
var points []*pb.Point
for i := 0; i < pointCount; i++ {
  points = append(points, randomPoint(r))
}
log.Printf("Traversing %d points.", len(points))
stream, err := client.RecordRoute(context.Background())
if err != nil {
  log.Fatalf("%v.RecordRoute(_) = _, %v", client, err)
}
for _, point := range points {
  if err := stream.Send(point); err != nil {
    log.Fatalf("%v.Send(%v) = %v", stream, point, err)
  }
}
reply, err := stream.CloseAndRecv()
if err != nil {
  log.Fatalf("%v.CloseAndRecv() got error %v, want %v", stream, err, nil)
}
log.Printf("Route summary: %v", reply)
```

The `RouteGuide_RecordRouteClient` has a `Send()` method that we can use to send
requests to the server. Once we’ve finished writing our client’s requests to the
stream using `Send()`, we need to call `CloseAndRecv()` on the stream to let
gRPC know that we’ve finished writing and are expecting to receive a response.
We get our RPC status from the err returned from `CloseAndRecv()`. If the status
is nil, then the first return value from `CloseAndRecv()` will be a valid server
response.

#### Bidirectional streaming RPC 

Finally, let’s look at our bidirectional streaming RPC `RouteChat()`. As in the
case of `RecordRoute`, we only pass the method a context object and get back a
stream that we can use to both write and read messages. However, this time we
return values via our method’s stream while the server is still writing messages
to their message stream.

```go
stream, err := client.RouteChat(context.Background())
waitc := make(chan struct{})
go func() {
  for {
    in, err := stream.Recv()
    if err == io.EOF {
      // read done.
      close(waitc)
      return
    }
    if err != nil {
      log.Fatalf("Failed to receive a note : %v", err)
    }
    log.Printf("Got message %s at point(%d, %d)", in.Message, in.Location.Latitude, in.Location.Longitude)
  }
}()
for _, note := range notes {
  if err := stream.Send(note); err != nil {
    log.Fatalf("Failed to send a note: %v", err)
  }
}
stream.CloseSend()
<-waitc
```

The syntax for reading and writing here is very similar to our client-side
streaming method, except we use the stream’s `CloseSend()` method once we’ve
finished our call. Although each side will always get the other’s messages in
the order they were written, both the client and server can read and write in
any order — the streams operate completely independently.

## Try it out

Execute the following commands from the working directory:

1. Run the server:

```sh
cd server
go run .
```

2. Run the client from another terminal:

```sh
cd client
go run .
```

You’ll see output like this:

```
Looking for features within lo:<latitude:400000000 longitude:-750000000 > hi:<latitude:420000000 longitude:-730000000 >
name:"Patriots Path, Mendham, NJ 07945, USA" location:<latitude:407838351 longitude:-746143763 >
...
name:"3 Hasta Way, Newton, NJ 07860, USA" location:<latitude:410248224 longitude:-747127767 >
Traversing 56 points.
Route summary: point_count:56 distance:497013163
Got message First message at point(0, 1)
Got message Second message at point(0, 2)
Got message Third message at point(0, 3)
Got message First message at point(0, 1)
Got message Fourth message at point(0, 1)
Got message Second message at point(0, 2)
Got message Fifth message at point(0, 2)
Got message Third message at point(0, 3)
Got message Sixth message at point(0, 3)
```
> [!NOTE]
> We’ve omitted timestamps from the client and server trace output shown in this page

## What’s next

* Learn how gRPC works in [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/) and [Core concepts](https://grpc.io/docs/what-is-grpc/core-concepts/).
* Work through the [Basics tutorial](https://grpc.io/docs/languages/go/basics/).
* Explore the [API reference](https://grpc.io/docs/languages/go/api).
