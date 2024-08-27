# Getting Started with gRPC-Java

|||
| :---- | :---- |
| Summary | Get hands-on with gRPC for Java in this interactive codelab\!  Perfect for Java developers new to gRPC, those seeking a refresher, or anyone building distributed systems.  No prior gRPC experience needed\! |
| **URL** | devsite/codelabs/docs |

# Before you begin

## **Prerequisites**

* [JDK](https://jdk.java.net/) version 8 or higher
    * We recommend [openjdk temurin v21](https://cloud.google.com/java/docs/setup\#install\_a\_jdk\_java\_development\_kit)
* Clone the [grpc codelab repo](https://github.com/grpc-ecosystem/grpc-codelabs.git)

```
git clone https://github.com/grpc-ecosystem/grpc-codelabs.git
```

## **What you’ll learn**

* Get hands-on with gRPC for Java in this interactive codelab\! Perfect for Java developers new to gRPC, those seeking a refresher, or anyone building distributed systems. No prior gRPC experience needed\!
* Build a complete gRPC service from scratch, learning:
    * Protocol Buffers (protobuf): Define service contracts & data.
    * gRPC Code Generation: Auto-generate Java code.
    * Client/Server Communication: Implement seamless interactions.
    * Testing & Debugging: Ensure reliability & correctness.
* You'll gain:
    * A working gRPC service in Java.
    * Hands-on experience with Protocol Buffers and code generation.
    * Skills to design, build, & test gRPC clients and servers.
    * A strong foundation in gRPC for real-world projects.

## **What you’ll need**

* A computer with internet connection

---

# Setup

[Download](https://download-directory.github.io/?url=https%3A%2F%2Fgithub.com%2Fgrpc-ecosystem%2Fgrpc-codelabs%2Ftree%2Fmain%2Fcodelabs%2FGetting\_Started\_with\_gRPC\_Java) the codelab or Clone the codelab repo, if you haven’t yet done so.  
To download without using git:

* go to [https://github.com/grpc-ecosystem/grpc-codelabs.git](https://github.com/grpc-ecosystem/grpc-codelabs.git)
* click on \`\<\> Code\`
* select \`Download ZIP\`

Change directory to `codelabs/grpc-java-getting-started/start_here`

Tip: For complete versions of each of the files we are editing, look in the `../complete` directory

# Define proto

Duration: 5:00

Our first step is to define the gRPC *service* and the method *request* and *response* types using [protocol buffers](https://protobuf.dev/overview).

Let’s create a `route_guide.proto` file.  
We’ve given you some boiler plate to start with in `src/main/proto/routeguide/route_guide.proto`

Since we’re generating Java code in this example, we’ve specified a `java_package` file option and a name for the Java class in our `.proto`:

```
option java_package = "io.grpc.examples.routeguide";
option java_outer_classname = "RouteGuideProto";
```

## **Define proto Message**

Our `.proto` file contains protocol buffer message type definitions for all the request and response types used in our service methods.

Let’s define the `Point` message type (`a latitude and a longitude, both multiplied by 10**7`):

```
// Points are represented as latitude-longitude pairs in the E7 representation(degrees times 10**7).
// Latitudes should be in the range +/- 90 degrees.
// Longitude should be in the range +/- 180 degrees.

message Point {
  int32 latitude = 1;
  int32 longitude = 2;
}
```

Let’s also define the `Feature` message type (`A feature names something at a given point`):

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

To define a service, you specify a named service in your `.proto` file:

```
service RouteGuide {
  // Definition of the service goes here
}
```

## **Define RPC Method**

Then you define `rpc` methods inside your service definition, specifying their request and response types.  In this section of the codelab, let’s define

* A Unary RPC method \- A *simple RPC* where the client sends a request to the server using the stub and waits for a response to come back, just like a normal function call.

```
// Obtains the feature at a given position.
rpc GetFeature(Point) returns (Feature) {}
```

---

# Generating client and server code

Next we need to generate the gRPC client and server interfaces from our .proto service definition. We do this using the protocol buffer compiler `protoc` with a special gRPC Java plugin. You need to use the [proto3](https://github.com/google/protobuf/releases) compiler (which supports both proto2 and proto3 syntax) in order to generate gRPC services.

When using Gradle or Maven, the protoc build plugin can generate the necessary code as part of the build. You can refer to the [grpc-java README](https://github.com/grpc/grpc-java/blob/master/README.md) for how to generate code from your own `.proto` files.

We have provided Gradle configuration.


| Note: You may need to do `chmod +x ../gradlew` if you downloaded a zip instead of doing `git clone`. |
| :---- |

From the `start_here` directory enter

```
../gradlew build
```

The following classes are generated from our service definition:

* `Feature.java`, `Point.java` and others which contain all the protocol buffer code to populate, serialize, and retrieve our request and response message types.
* `RouteGuideGrpc.java` which contains (along with some other useful code):
    * a base class for `RouteGuide` servers to implement, `RouteGuideGrpc.RouteGuideImplBase`, with all the methods defined in the `RouteGuide` service
    * Stub classes for clients to use

---

# Creating the server

Duration: 5:00

First let’s look at how we create a `RouteGuide` server. There are two parts to making our `RouteGuide` service do its job:

* Implementing the service interface generated from our service definition: doing the actual “work” of our service.
* Running a gRPC server to listen for requests from clients and dispatch them to the right service implementation.

## **Implementing RouteGuide**

As you can see, our server has a `RouteGuideService` class that extends the generated `RouteGuideGrpc.RouteGuideImplBase` abstract class:

```java
private static class RouteGuideService extends RouteGuideGrpc.RouteGuideImplBase {
...
}
```

We have provided the following 2 files for initializing your server with features  
`./src/main/java/io/grpc/examples/routeguide/RouteGuideUtil.java`  
`./src/main/resources/io/grpc/examples/routeguide/route_guide_db.json`

Let us look into a simple RPC implementation in detail

### **Unary RPC**

`RouteGuideService` implements all our service methods, in this case it is just `GetFeature()`, which just gets a `Point` from the client and returns the corresponding feature information from its database in a `Feature`.  
We include `checkFeature`.  The most important aspect is creating a `Feature` object

**Feature.newBuilder().setName("").setLocation(location).build();**| // Creates a feature.
---|---

```java
@Override
public void getFeature(Point request, StreamObserver<Feature> responseObserver) {
  responseObserver.onNext(checkFeature(request));
  responseObserver.onCompleted();
}
```

The `getFeature()` method takes two parameters:

* `Point`: the request
* `StreamObserver<Feature>`: a response observer, which is a special interface for the server to call with its response.

To return our response to the client and complete the call:

1. We construct and populate a `Feature` response object to return to the client, as specified in our service definition. In this example, we do this in a separate private `checkFeature()` method.
2. We use the response observer’s `onNext()` method to return the `Feature`.
3. We use the response observer’s `onCompleted()` method to specify that we’ve finished dealing with the RPC.

---

# Starting the gRPC server

Duration: 5:00

Once we’ve implemented all our service methods, we need to start up a gRPC server so that clients can actually use our service. We include in our boilerplate the creation of the ServerBuilder object:

`ServerBuilder.forPort(port), port, RouteGuideUtil.parseFeatures(featureFile)`

We build the service in the constructor:

1. Specify the port we want to use to listen for client requests using the builder’s `forPort()` method (it will use the wildcard address).
2. Create an instance of our service implementation class `RouteGuideService` and pass it to the builder’s `addService()` method.
3. Call `build()` on the builder to create an RPC server for our service.

The following snippet shows how we create a `ServerBuilder` object.


```java
     this(Grpc.newServerBuilderForPort(port, InsecureServerCredentials.create()),
     port, RouteGuideUtil.parseFeatures(featureFile));
```

The following snippet shows how we create a server object for our `RouteGuide` service.

```java
/** Create a RouteGuide server using serverBuilder as a base and features as data. */
public RouteGuideServer(ServerBuilder<?> serverBuilder, int port, Collection<Feature> features) {
  this.port = port;
  server = serverBuilder.addService(new RouteGuideService(features))
      .build();
}
```

Implement a start method that calls `start` on the server we created above

```java
public void start() throws IOException {
  server.start();
  logger.info("Server started, listening on " + port);
}
```

Implement a method to wait for the server to complete so it doesn’t immediately exit.

```java
/** Await termination on the main thread since the grpc library uses daemon threads. */
private void blockUntilShutdown() throws InterruptedException {
  if (server != null) {
    server.awaitTermination();
  }
}
```

As you can see, we build and start our server using a `ServerBuilder`.

In the main method we

1. Create a `RouteGuideServer` instance
2. Call `start()` to activate an RPC server for our service.
3. Wait for the service to be stopped by calling `blockUntilShutdown()`

```java
  public static void main(String[] args) throws Exception {
    RouteGuideServer server = new RouteGuideServer(8980);
    server.start();
    server.blockUntilShutdown();
  }
```

---

# Creating the client

Duration: 5:00

In this section, we’ll look at creating a client for our `RouteGuide` service. You can see our complete example client code in ../complete/src/main/java/io/grpc/examples/routeguide/RouteGuideClient.java

## **Instantiating a stub**

To call service methods, we first need to create a *stub*.  There are two types of stubs, but we only need to use the blocking one for this codelab.  The 2 types are:

* a *blocking/synchronous* stub: this means that the RPC call waits for the server to respond, and will either return a response or raise an exception.
* a *non-blocking/asynchronous* stub that makes non-blocking calls to the server, where the response is returned asynchronously. You can make certain types of streaming calls only by using the asynchronous stub.

First we need to create a gRPC *channel* and then use the channel to create our stub.

We could have used a `ManagedChannelBuilder` directly to create the channel.

```java
ManagedChannelBuilder.forAddress(host, port).usePlaintext().build
```

But let’s use a utility method that takes a string with `hostname:port`

```java
Grpc.newChannelBuilder(target, InsecureChannelCredentials.create()).build();
```

Now we can use the channel to create our blocking stub.  For this codelab, we only have blocking RPCs, so we use the `newBlockingStub` method provided in the `RouteGuideGrpc` class we generated from our `.proto`.

```java
blockingStub = RouteGuideGrpc.newBlockingStub(channel);
```

## **Calling service methods**

Now let’s look at how we call our service methods.

### **Simple RPC**

Calling the simple RPC `GetFeature` is nearly as straightforward as calling a local method.

We create and populate a request protocol buffer object (in our case `Point`), pass it to the `getFeature()` method on our blocking stub, and get back a `Feature`.

If an error occurs, it is encoded as a `Status`, which we can obtain from the `StatusRuntimeException`.

```java
Point request = Point.newBuilder().setLatitude(lat).setLongitude(lon).build();

Feature feature;
try {
  feature = blockingStub.getFeature(request);
} catch (StatusRuntimeException e) {
  logger.log(Level.WARNING, "RPC failed: {0}", e.getStatus());
  return;
}
```

The boilerplate logs a message containing the contents based on whether or not there was a feature at the specified point.

---

# Try it out\!

Duration: 2:00

## **To build the codelab**

1. From the `start_here` directory:

```
$ ../gradlew installDist
```

This will compile your code, package it in a jar and create the scripts that run the example.  They will be created in the `build/install/start_here/bin/` directory. The scripts are: `route-guide-server` and `route-guide-client`.

The server needs to be running before starting the client.

2. Run the server:

```
$ ./build/install/start_here/bin/route-guide-server
```

3. Run the client:

```
$ ./build/install/start_here/bin/route-guide-client
```

---

# What’s next

* Do the streaming code lab gRPC Java Streaming (../../grpc-java-streaming)
* Learn how gRPC works in [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/) and [Core concepts](https://grpc.io/docs/what-is-grpc/core-concepts/).
* Explore the [API reference](https://grpc.io/docs/languages/java/api).
