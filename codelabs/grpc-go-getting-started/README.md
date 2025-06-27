# Getting Started with gRPC-Go

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
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
```

* Update your PATH so that the protoc compiler can find the plugins:

```sh
export PATH="$PATH:$(go env GOPATH)/bin"
```

* Use [this as a starting point](https://download-directory.github.io/?url=https%3A%2F%2Fgithub.com%2Fgrpc-ecosystem%2Fgrpc-codelabs%2Ftree%2Fmain%2Fcodelabs%2Fgrpc-go-getting-started%2Fstart_here) for this codelab.

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

### Define the RouteGuide service

To define a service, you specify a named service in your `.proto` file:

```proto
service RouteGuide {
  // Definition of the service goes here
}
```

### Define the RPC method in the service

Then you define `rpc` methods inside your service definition, specifying their
request and response types.  In this section of the codelab, let’s define
`GetFeature` method that returns the named `Feature` for the given `Point.`

This would be Unary RPC method \- A *simple RPC* where the client sends a
request to the server using the stub and waits for a response to come back, just
like a normal function call.

```proto
// Obtains the feature at a given position.
rpc GetFeature(Point) returns (Feature) {}
```

> [!TIP]
>  For the complete .proto file, see [routeguide/route_guide.proto](/grpc-go-getting-started/completed/routeguide/route_guide.proto).

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

func (s *routeGuideServer) GetFeature(ctx context.Context, point *pb.Point) (*pb.Feature, error) {
        ...
}
```

Let us look into the RPC implementation in detail

#### Unary RPC

The `routeGuideServer` implements all our service methods. Let’s look at
`GetFeature` which just gets a `Point` from the client and returns the
corresponding feature information from its database in a `Feature`.

```go
func (s *routeGuideServer) GetFeature(ctx context.Context, point *pb.Point) (*pb.Feature, error) {
  for _, feature := range s.savedFeatures {
    if proto.Equal(feature.Location, point) {
      return feature, nil
    }
  }
  // No feature was found, return an unnamed feature
  return &pb.Feature{Location: point}, nil
}

```

The method is passed a context object for the RPC and the client’s `Point`
protocol buffer request. It returns a `Feature` protocol buffer object with the
response information and an `error`. In the method we populate the `Feature`
with the appropriate information, and then `return` it along with a nil error to
tell gRPC that we’ve finished dealing with the RPC and that the `Feature` can be
returned to the client.

## Starting the server

Once we’ve implemented all our methods, we also need to start up a gRPC server
so that clients can actually use our service. The following snippet shows how we
do this for our `RouteGuide` service:

> [!NOTE]
>  port can be configured by passing in `port` flag. Defaults to `50051`

```go
lis, err := net.Listen("tcp", fmt.Sprintf("localhost:%d", *port))
if err != nil {
  log.Fatalf("failed to listen: %v", err)
}
var opts []grpc.ServerOption
grpcServer := grpc.NewServer(opts...)

s := &routeGuideServer{}
s.loadFeatures()
pb.RegisterRouteGuideServer(grpcServer, s)
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

#### Simple RPC

Calling the simple RPC `GetFeature` is nearly as straightforward as calling a local method.

```go
feature, err := client.GetFeature(context.Background(), &pb.Point{Latitude: 409146138, Longitude: -746188906})
if err != nil {
    ...
}
```

As you can see, we call the method on the stub we got earlier. In our method
parameters we create and populate a request protocol buffer object (in our case
`Point`). We also pass a `context.Context` object which lets us change our RPC’s
behavior if necessary, such as time-out/cancel an RPC in flight. If the call
doesn’t return an error, then we can read the response information from the
server from the first return value.

```go
log.Println(feature)
```

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
Getting feature for point (409146138, -746188906)
name:"Berkshire Valley Management Area Trail, Jefferson, NJ, USA" location:<latitude:409146138 longitude:-746188906 >
Getting feature for point (0, 0)
location:<>
```
> [!NOTE]
> We’ve omitted timestamps from the client and server trace output shown in this page

## What’s next

* Learn how gRPC works in [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/) and [Core concepts](https://grpc.io/docs/what-is-grpc/core-concepts/).
* Work through the [Basics tutorial](https://grpc.io/docs/languages/go/basics/).
* Explore the [API reference](https://grpc.io/docs/languages/go/api).
