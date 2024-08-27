# Getting Started with gRPC-Java \- Streaming

|  |                                                                                                                                                                                                                                                  |
|:--------|:-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Summary| Learn to develop and use streaming RPCs with gRPC for Java in this interactive codelab\! <br/>Perfect for Java developers new to gRPC, those seeking a refresher, or anyone building distributed systems. <br/>No prior gRPC experience needed\. |     |
| **URL** | devsite/codelabs/docs                                                                                                                                                                                                                            |

[TOC] 

# Before you begin 

If you are here after completing Getting Started with gRPC-Java codelab, you can skip this step

## **Prerequisites** 

* [JDK](https://jdk.java.net/) version 7 or higher
* Clone the [grpc codelab repo](https://github.com/grpc-ecosystem/grpc-codelabs.git)

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

Clone the codelab repo if you haven’t yet done so

```
git clone https://github.com/grpc-ecosystem/grpc-codelabs.git
```

You can also go to the above link, click on `<> Code` and then select `Download ZIP`

Change directory to `codelabs/Getting_Started_with_gRPC_Java_Streaming/start_here`

Tip: For complete versions of each of the files we are editing, look in the `../complete` directory 

# Define proto 

Duration: 5:00

Our first step is to define the gRPC *service* and the method *request* and *response* types using [protocol buffers](https://protobuf.dev/overview).

Tip: For the complete .proto file, see [grpc-java/examples/src/main/proto/route\_guide.proto.](https://github.com/grpc/grpc-java/blob/master/examples/src/main/proto/route\_guide.proto)

Let’s create a `route_guide.proto` file.

Since we’re generating Java code in this example, we’ve specified a `java_package` file option in our `.proto`:

```
option java_package = "io.grpc.examples.routeguide";
```

## **Define proto Message** 

Our `.proto` file contains protocol buffer message type definitions for all the request and response types used in our service methods \- the message types have been specified for you as has the portion of the RouteGuide service from the [Getting\_Started\_with\_gRPC\_Java codelab](https://docs.google.com/document/d/1gevX49Yqnp-Ae2ukcJdGLpD33js\_OK9OokrrsM2vY7M/).

## **Define RouteGuide service** 

To define a service, you specify a named service in your `.proto` file:

```
service RouteGuide {
  // Definition of the service goes here
}
```

## **Define RPC Method** 

Then you define `rpc` methods inside your service definition, specifying their request and response types.  In this section of the codelab, let’s define

* A *server-side streaming RPC* where the client sends a request to the server and gets a stream to read a sequence of messages back. The client reads from the returned stream until there are no more messages. As you can see in our example, you specify a server-side streaming method by placing the `stream` keyword before the *response* type.

```
// Obtains the Features available within the given Rectangle.  Results
// are streamed rather than returned at once (e.g. in a response message
// with a repeated field), as the rectangle may cover a large area and 
// contain a huge number of features.

rpc ListFeatures(Rectangle) returns (stream Feature) {}
```

* A *client-side streaming RPC* where the client writes a sequence of messages and sends them to the server, again using a provided stream. Once the client has finished writing the messages, it waits for the server to read them all and return its response. You specify a client-side streaming method by placing the `stream` keyword before the *request* type.

```
// Accepts a stream of Points on a route being traversed, returning a
// RouteSummary when traversal is completed.
rpc RecordRoute(stream Point) returns (RouteSummary) {}
```

* A *bidirectional streaming RPC* where both sides send a sequence of messages using a read-write stream. The two streams operate independently, so clients and servers can read and write in whatever order they like: for example, the server could wait to receive all the client messages before writing its responses, or it could alternately read a message then write a message, or some other combination of reads and writes. The order of messages in each stream is preserved. You specify this type of method by placing the `stream` keyword before both the request and the response.

```
// Accepts a stream of RouteNotes sent while a route is being traversed,
// while receiving other RouteNotes (e.g. from other users).
rpc RouteChat(stream RouteNote) returns (stream RouteNote) {}
```

---

# Generating client and server code  

Next we need to generate the gRPC client and server interfaces from our `.proto` service definition. We do this using the protocol buffer compiler `protoc` with a special gRPC Java plugin. You need to use the [proto3](https://github.com/google/protobuf/releases) compiler (which supports both proto2 and proto3 syntax) in order to generate gRPC services.

When using Gradle or Maven, the protoc build plugin can generate the necessary code as part of the build. You can refer to the [grpc-java README](https://github.com/grpc/grpc-java/blob/master/README.md) for how to generate code from your own `.proto` files if you don’t want to just rely on the Gradle plugin.

The following classes are generated from our service definition:

* One for each message type: `Feature.java`, `Rectangle.java`, … which contain all of the protocol buffer code to populate, serialize, and retrieve our request and response message types.
* `RouteGuideGrpc.java` which contains (along with some other useful code):
    * a base class for RouteGuide servers to implement, `RouteGuideGrpc.RouteGuideImplBase`, with all the methods defined in the `RouteGuide` service
    * Stub classes for clients to use

---

# Creating the server 

Duration: 5:00

First let’s look at how we create a `RouteGuide` server. There are two parts to making our `RouteGuide` service do its job:

* Implementing the service interface generated from our service definition: doing the actual “work” of our service.
* Running a gRPC server to listen for requests from clients and dispatch them to the right service implementation.

## **Implementing RouteGuide** 

We need to implement the generated `RouteGuideService` interface:

```java

public void listFeatures(Rectangle request, StreamObserver<Feature> responseObserver) {
        ...
}

public StreamObserver<Point> recordRoute(final StreamObserver<RouteSummary> responseObserver) {

        ...
}

public StreamObserver<RouteNote> routeChat(final StreamObserver<RouteNote> responseObserver) {

        ...
}
```

Let us look into each RPC implementation in detail

### **Server-side streaming RPC**  

###  

Next let’s look at one of our streaming RPCs. `ListFeatures` is a server-side streaming RPC, so we need to send back multiple `Features` to our client.

```java
private final Collection<Feature> features;

@Override
public void listFeatures(Rectangle request, StreamObserver<Feature> responseObserver) {
  int left = min(request.getLo().getLongitude(), request.getHi().getLongitude());
  int right = max(request.getLo().getLongitude(), request.getHi().getLongitude());
  int top = max(request.getLo().getLatitude(), request.getHi().getLatitude());
  int bottom = min(request.getLo().getLatitude(), request.getHi().getLatitude());

  for (Feature feature : features) {
    if (!RouteGuideUtil.exists(feature)) {
      continue;
    }

    int lat = feature.getLocation().getLatitude();
    int lon = feature.getLocation().getLongitude();
    if (lon >= left && lon <= right && lat >= bottom && lat <= top) {
      responseObserver.onNext(feature);
    }
  }
  responseObserver.onCompleted();
}
```

Like the simple RPC, this method gets a request object (the `Rectangle` in which our client wants to find `Features`) and a `StreamObserver` response observer.

This time, we get as many `Feature` objects as we need to return to the client (in this case, we select them from the service’s feature collection based on whether they’re inside our request `Rectangle`), and write them each in turn to the response observer using its `onNext()` method. Finally, as in our simple RPC, we use the response observer’s `onCompleted()` method to tell gRPC that we’ve finished writing responses.

### **Client-side streaming RPC**  

Now let’s look at something a little more complicated: the client-side streaming method `RecordRoute()`, where we get a stream of `Points` from the client and return a single `RouteSummary` with information about their trip.

```java
@Override
public StreamObserver<Point> recordRoute(final StreamObserver<RouteSummary> responseObserver) {
  return new StreamObserver<Point>() {
    int pointCount;
    int featureCount;
    int distance;
    Point previous;
    long startTime = System.nanoTime();

    @Override
    public void onNext(Point point) {
      pointCount++;
      if (RouteGuideUtil.exists(checkFeature(point))) {
        featureCount++;
      }
      // For each point after the first, add the incremental distance from the previous point
      // to the total distance value.
      if (previous != null) {
        distance += calcDistance(previous, point);
      }
      previous = point;
    }

    @Override
    public void onError(Throwable t) {
      logger.log(Level.WARNING, "Encountered error in recordRoute", t);
    }

    @Override
    public void onCompleted() {
      long seconds = NANOSECONDS.toSeconds(System.nanoTime() - startTime);
      responseObserver.onNext(RouteSummary.newBuilder().setPointCount(pointCount)
          .setFeatureCount(featureCount).setDistance(distance)
          .setElapsedTime((int) seconds).build());
      responseObserver.onCompleted();
    }
  };
}
```

As you can see, like the previous method types our method gets a `StreamObserver` `responseObserver` parameter, but this time it returns a `StreamObserver` for the client to write its `Points`.

In the method body we instantiate an anonymous `StreamObserver` to return, in which we:

* Override the `onNext()` method to get features and other information each time the client writes a Point to the message stream.
* Override the `onCompleted()` method (called when the *client* has finished writing messages) to populate and build our `RouteSummary`. We then call our method’s own response observer’s `onNext()` with our `RouteSummary`, and then call its `onCompleted()` method to finish the call from the server side.

### **Bidirectional streaming RPC**  

Finally, let’s look at our bidirectional streaming RPC `RouteChat()`.

```java
@Override
public StreamObserver<RouteNote> routeChat(final StreamObserver<RouteNote> responseObserver) {
  return new StreamObserver<RouteNote>() {
    @Override
    public void onNext(RouteNote note) {
      List<RouteNote> notes = getOrCreateNotes(note.getLocation());

      // Respond with all previous notes at this location.
      for (RouteNote prevNote : notes.toArray(new RouteNote[0])) {
        responseObserver.onNext(prevNote);
      }

      // Now add the new note to the list
      notes.add(note);
    }

    @Override
    public void onError(Throwable t) {
      logger.log(Level.WARNING, "Encountered error in routeChat", t);
    }

    @Override
    public void onCompleted() {
      responseObserver.onCompleted();
    }
  };
}
```

As with our client-side streaming example, we both get and return a `StreamObserver`, except this time we return values via our method’s response observer while the client is still writing messages to *their* message stream. The syntax for reading and writing here is exactly the same as for our client-streaming and server-streaming methods. Although each side will always get the other’s messages in the order they were written, both the client and server can read and write in any order — the streams operate completely independently.

---

# Starting the server 

Duration: 5:00

Once we’ve implemented all our methods, we also need to start up a gRPC server so that clients can actually use our service. The following snippet shows how we do this for our `RouteGuide` service:

```java
public RouteGuideServer(int port, URL featureFile) throws IOException {
  this(ServerBuilder.forPort(port), port, RouteGuideUtil.parseFeatures(featureFile));
}

/** Create a RouteGuide server using serverBuilder as a base and features as data. */
public RouteGuideServer(ServerBuilder<?> serverBuilder, int port, Collection<Feature> features) {
  this.port = port;
  server = serverBuilder.addService(new RouteGuideService(features))
      .build();
}
```

```java
public void start() throws IOException {
  server.start();
  logger.info("Server started, listening on " + port);
}
```

As you can see, we build and start our server using a `ServerBuilder`.

To do this, we:

1. Specify the address and port we want to use to listen for client requests using the builder’s `forPort()` method.
2. Create an instance of our service implementation class `RouteGuideService` and pass it to the builder’s `addService()` method.
3. Call `build()` and `start()` on the builder to create and start an RPC server for our service.

Since the ServerBuilder already incorporates the port, the only reason we pass a port is to use it for logging.

---

# Creating the client 

Duration: 5:00

In this section, we’ll look at creating a client for our `RouteGuide` service. You can see our complete example client code in `../complete/src/main/java/io/grpc/complete/routeguide/RouteGuideClient.java`.

## **Instantiating a stub**  

To call service methods, we first need to create a *stub*, or rather, two stubs:

* a *blocking/synchronous* stub: this means that the RPC call waits for the server to respond, and will either return a response or raise an exception.
* a *non-blocking/asynchronous* stub that makes non-blocking calls to the server, where the response is returned asynchronously. You can make certain types of streaming calls only by using an asynchronous stub.

First we need to create a gRPC *channel* for our stub, specifying the server address and port we want to connect to:

```java
public RouteGuideClient(String host, int port) {
  this(ManagedChannelBuilder.forAddress(host, port).usePlaintext());
}

/** Construct client for accessing RouteGuide server using the existing channel. */
public RouteGuideClient(ManagedChannelBuilder<?> channelBuilder) {
  channel = channelBuilder.build();
  blockingStub = RouteGuideGrpc.newBlockingStub(channel);
  asyncStub = RouteGuideGrpc.newStub(channel);
}
```

We use a `ManagedChannelBuilder` to create the channel.

Now we can use the channel to create our stubs using the `newStub` and `newBlockingStub` methods provided in the `RouteGuideGrpc` class we generated from our `.proto`.

```java
blockingStub = RouteGuideGrpc.newBlockingStub(channel);
asyncStub = RouteGuideGrpc.newStub(channel);
```

Remember, if it’s not blocking, it’s async

## **Calling service methods**  

Now let’s look at how we call our service methods. Note that any RPCs created from the blocking stub will operate in a blocking/synchronous mode, which means that the RPC call waits for the server to respond, and will either return a response or an error.

### **Server-side streaming RPC**  

Next, let’s look at a server-side streaming call to `ListFeatures`, which returns a stream of geographical `Feature`:

```java
Rectangle request = Rectangle.newBuilder()
             .setLo(Point.newBuilder().setLatitude(lowLat).setLongitude(lowLon).build())
        .setHi(Point.newBuilder().setLatitude(hiLat).setLongitude(hiLon).build()).build();

Iterator<Feature> features;
try {
  features = blockingStub.listFeatures(request);
} catch (StatusRuntimeException e) {
  logger.log(Level.WARNING, "RPC failed: {0}", e.getStatus());
  return;
}
```

As you can see, it’s very similar to the simple unary RPC we looked at in the Getting\_Started\_With\_gRPC\_Java codelab, except instead of returning a single `Feature`, the method returns an `Iterator` that the client can use to read all the returned `Features`.

### **Client-side streaming RPC** 

Now for something a little more complicated: the client-side streaming method `RecordRoute`, where we send a stream of `Points` to the server and get back a single `RouteSummary`. For this method we need to use the **asynchronous** stub. If you’ve already read [Creating the server](https://grpc.io/docs/languages/java/basics/\#server), some of this may look very familiar \- asynchronous streaming RPCs are implemented in a similar way on both sides.

```java
public void recordRoute(List<Feature> features, int numPoints) throws InterruptedException {
  info("*** RecordRoute");
  final CountDownLatch finishLatch = new CountDownLatch(1);
  StreamObserver<RouteSummary> responseObserver = new StreamObserver<RouteSummary>() {

    @Override
    public void onNext(RouteSummary summary) {
      info("Finished trip with {0} points. Passed {1} features. "
          + "Travelled {2} meters. It took {3} seconds.", summary.getPointCount(),
          summary.getFeatureCount(), summary.getDistance(), summary.getElapsedTime());
    }

    @Override
    public void onError(Throwable t) {
      Status status = Status.fromThrowable(t);
      logger.log(Level.WARNING, "RecordRoute Failed: {0}", status);
      finishLatch.countDown();
    }

    @Override
    public void onCompleted() {
      info("Finished RecordRoute");
      finishLatch.countDown();
    }
  };

  StreamObserver<Point> requestObserver = asyncStub.recordRoute(responseObserver);
  try {
    // Send numPoints points randomly selected from the features list.
    Random rand = new Random();
    for (int i = 0; i < numPoints; ++i) {
      int index = rand.nextInt(features.size());
      Point point = features.get(index).getLocation();
      info("Visiting point {0}, {1}", RouteGuideUtil.getLatitude(point),
          RouteGuideUtil.getLongitude(point));
      requestObserver.onNext(point);
      // Sleep for a bit before sending the next one.
      Thread.sleep(rand.nextInt(1000) + 500);
      if (finishLatch.getCount() == 0) {
        // RPC completed or errored before we finished sending.
        // Sending further requests won't error, but they will just be thrown away.
        return;
      }
    }
  } catch (RuntimeException e) {
    // Cancel RPC
    requestObserver.onError(e);
    throw e;
  }
  // Mark the end of requests
  requestObserver.onCompleted();

  // Receiving happens asynchronously
  finishLatch.await(1, TimeUnit.MINUTES);
}
```

As you can see, to call this method we need to create a `StreamObserver`, which implements a special interface for the server to call with its `RouteSummary` response. In our `StreamObserver` we:

* Override the `onNext()` method to print out the returned information when the server writes a `RouteSummary` to the message stream.
* Override the `onCompleted()` method (called when the *server* has completed the call on its side) to reduce a `CountDownLatch` so that we can check to see if the server has finished writing.

We then pass the `StreamObserver` to the asynchronous stub’s `recordRoute()` method and get back our own `StreamObserver` request observer to write our `Points` to send to the server. Once we’ve finished writing points, we use the request observer’s `onCompleted()` method to tell gRPC that we’ve finished writing on the client side. Once we’re done, we check our `CountDownLatch` to see if the server has completed on its side.

### **Bidirectional streaming RPC**  

Finally, let’s look at our bidirectional streaming RPC `RouteChat()`.

```java
public void routeChat() throws Exception {
  info("*** RoutChat");
  final CountDownLatch finishLatch = new CountDownLatch(1);
  StreamObserver<RouteNote> requestObserver =
      asyncStub.routeChat(new StreamObserver<RouteNote>() {

        @Override
        public void onNext(RouteNote note) {
          info("Got message \"{0}\" at {1}, {2}", note.getMessage(), note.getLocation()
              .getLatitude(), note.getLocation().getLongitude());
        }

        @Override
        public void onError(Throwable t) {
          Status status = Status.fromThrowable(t);
          logger.log(Level.WARNING, "RouteChat Failed: {0}", status);
          finishLatch.countDown();
        }

        @Override
        public void onCompleted() {
          info("Finished RouteChat");
          finishLatch.countDown();
        }
      });

  try {
    RouteNote[] requests =
        {newNote("First message", 0, 0), newNote("Second message", 0, 1),
            newNote("Third message", 1, 0), newNote("Fourth message", 1, 1)};

    for (RouteNote request : requests) {
      info("Sending message \"{0}\" at {1}, {2}", request.getMessage(), request.getLocation()
          .getLatitude(), request.getLocation().getLongitude());
      requestObserver.onNext(request);
    }
  } catch (RuntimeException e) {
    // Cancel RPC
    requestObserver.onError(e);
    throw e;
  }
  // Mark the end of requests
  requestObserver.onCompleted();

  // Receiving happens asynchronously
  finishLatch.await(1, TimeUnit.MINUTES);
}
```

As with our client-side streaming example, we both get and return a `StreamObserver` response observer, except this time we send values via our method’s response observer while the server is still writing messages to *their* message stream. The syntax for reading and writing here is exactly the same as for our client-streaming method. Although each side will always get the other’s messages in the order they were written, both the client and server can read and write in any order — the streams operate completely independently.

---

# Try it out\!  

Duration: 2:00

## **To build the examples**

1. [Install gRPC Java library SNAPSHOT locally, including code generation plugin](https://github.com/grpc/grpc-java/blob/master/COMPILING.md) (Only need this step for non-released versions, e.g. master HEAD).
2. From the `codelabs/Getting_Started_with_gRPC_Java_Streaming/start_here` directory:

```
$ ../gradlew installDist
```

This creates the scripts route-guide-server and route-guide-client in the `build/install/start_here/bin/` directory. The server must be running before starting the client.

3. Run the server:

```
$ ./build/install/start_here/bin/route-guide-server
```

4. Run the client:

```
$ ./build/install/start_here/bin/route-guide-client
```

---

# What’s next 

* Learn how gRPC works in [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/) and [Core concepts](https://grpc.io/docs/what-is-grpc/core-concepts/).
* Explore the [API reference](https://grpc.io/docs/languages/java/api).

---

# Surveys 

Duration: 1:00  