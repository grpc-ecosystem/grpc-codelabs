package io.grpc.examples.routeguide;

import com.google.protobuf.Message;
import io.grpc.Channel;
import io.grpc.Grpc;
import io.grpc.InsecureChannelCredentials;
import io.grpc.ManagedChannel;
import io.grpc.Status;
import io.grpc.StatusRuntimeException;
import io.grpc.examples.routeguide.RouteGuideGrpc.RouteGuideBlockingStub;
import io.grpc.examples.routeguide.RouteGuideGrpc.RouteGuideStub; // For Phase 2
import io.grpc.stub.StreamObserver; 				  // For Phase 2
import java.io.IOException;
import java.util.List;
import java.util.concurrent.TimeUnit;
import java.util.logging.Level;
import java.util.logging.Logger;

/**
 * Sample client code that makes gRPC calls to the server.
 */
public class RouteGuideClient {
  private static final Logger logger = Logger.getLogger(RouteGuideClient.class.getName());

  private final RouteGuideBlockingStub blockingStub;
  private final RouteGuideStub asyncStub;

  /** Construct client stub for accessing RouteGuide server using the existing channel. */
  public RouteGuideClient(Channel channel) {
    /****************************************************************
     * Codelab Hint: Create a blocking stub and an async stub for your
     * service using the code generated from the proto file (RouteGuideGrpc)
     *
     blockingStub =
     asyncStub =
     ****************************************************************/
  }

  /**
   * Blocking unary call example.  Calls getFeature and prints the response.
   */
  public void getFeature(int lat, int lon) {
    info("*** GetFeature: lat={0} lon={1}", lat, lon);

    Point request = Point.newBuilder().setLatitude(lat).setLongitude(lon).build();

    Feature feature;
    try {
      /****************************************************************
       * Codelab Hint: Use the blocking stub to make an RPC call to getFeature
       ****************************************************************/
    } catch (StatusRuntimeException e) {
      warning("RPC failed: {0}", e.getStatus());
      return;
    }
    if (RouteGuideUtil.exists(feature)) {
      info("Found feature called \"{0}\" at {1}, {2}",
          feature.getName(),
          RouteGuideUtil.getLatitude(feature.getLocation()),
          RouteGuideUtil.getLongitude(feature.getLocation()));
    } else {
      info("Found no feature at {0}, {1}",
          RouteGuideUtil.getLatitude(feature.getLocation()),
          RouteGuideUtil.getLongitude(feature.getLocation()));
    }
  }

  /**
   * Blocking server-streaming example. Calls listFeatures with a rectangle of interest. Prints each
   * response feature as it arrives.
   * Requires using the async API.
   */
  public void listFeatures(int lowLat, int lowLon, int hiLat, int hiLon) {
    info("*** ListFeatures: lowLat={0} lowLon={1} hiLat={2} hiLon={3}", lowLat, lowLon, hiLat,
        hiLon);

    /****************************************************************
     * Codelab Hint: Create a Rectangle using Rectangle.newBuilder()
     *
     Rectangle request =
     ****************************************************************/

    Iterator<Feature> features;
    try {
      /****************************************************************
       * Codelab Hint: Retrieve the features using the blocking stub
       * and a listFeatures RPC call
       *
       features =
       ****************************************************************/

      for (int i = 1; features.hasNext(); i++) {
        Feature feature = features.next();
        info("Result #" + i + ": {0}", feature);
      }
    } catch (StatusRuntimeException e) {
      warning("RPC failed: {0}", e.getStatus());
    }
  }

  /**
   * Async client-streaming example. Sends {@code numPoints} randomly chosen points from {@code
   * features} with a variable delay in between. Prints the statistics when they are sent from the
   * server.
   */
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
        warning("RecordRoute Failed: {0}", Status.fromThrowable(t));
        finishLatch.countDown();
      }

      @Override
      public void onCompleted() {
        info("Finished RecordRoute");
        finishLatch.countDown();
      }
    };


    /****************************************************************
     * Codelab Hint: Start the recordRoute RPC using the asyncStub
     *   - Put the return value into requestObserver
     *
     StreamObserver<Point> requestObserver =
     ****************************************************************/
    try {
      // Send numPoints points randomly selected from the features list.
      for (int i = 0; i < numPoints; ++i) {
        int index = random.nextInt(features.size());
        Point point = features.get(index).getLocation();
        info("Visiting point {0}, {1}", RouteGuideUtil.getLatitude(point),
            RouteGuideUtil.getLongitude(point));

        /****************************************************************
         * Codelab Hint: send point to the server using the requestObserver
         *
         ****************************************************************/
        // Sleep for a bit before sending the next one.
        Thread.sleep(random.nextInt(1000) + 500);
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
    /****************************************************************
     * Codelab Hint: Mark the end of requests using the requestObserver
     *
     ****************************************************************/

    // Receiving happens asynchronously
    if (!finishLatch.await(1, TimeUnit.MINUTES)) {
      warning("recordRoute did not finish within 1 minutes");
    }
  }

  /**
   * Bi-directional example, which can only be asynchronous. Send some chat messages, and print any
   * chat messages that are sent from the server.
   */
  public CountDownLatch routeChat() {
    info("*** RouteChat");
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
            warning("RouteChat Failed: {0}", Status.fromThrowable(t));
            finishLatch.countDown();
          }

          @Override
          public void onCompleted() {
            info("Finished RouteChat");
            finishLatch.countDown();
          }
        });

    try {
      RouteNote[] requests = {
              newNote("First message", 0, 0),
              newNote("Second message", 0, 10_000_000),
              newNote("Third message", 10_000_000, 0),
              newNote("Fourth message", 10_000_000, 10_000_000)};

      for (RouteNote request : requests) {
        info("Sending message \"{0}\" at {1}, {2}", request.getMessage(), request.getLocation()
            .getLatitude(), request.getLocation().getLongitude());

        /****************************************************************
         * Codelab Hint: Send the request using the requestObserver
         ****************************************************************/
      }
    } catch (RuntimeException e) {
      // Cancel RPC
      /****************************************************************
       * Codelab Hint: Send the error to the requestObserver
       ****************************************************************/

      throw e;
    }

    /****************************************************************
     * Codelab Hint: Mark the end of requests using reuestObserver
     ****************************************************************/

    // return the latch while receiving happens asynchronously
    return finishLatch;
  }

  /** Issues several different requests and then exits. */
  public static void main(String[] args) throws InterruptedException {
    String target = "localhost:8980";
    if (args.length > 0) {
      if ("--help".equals(args[0])) {
        System.err.println("Usage: [target]");
        System.err.println("");
        System.err.println("  target  The server to connect to. Defaults to " + target);
        System.exit(1);
      }
      target = args[0];
    }

        /***************************************************************
         * Codelab Hint: create a channel using the target defined above
         *
         ManagedChannel channel =
         ***************************************************************/

    try {
      // Create a client instance
      RouteGuideClient client = new RouteGuideClient(channel);

      // Looking for features between 40, -75 and 42, -73.
      client.listFeatures(400000000, -750000000, 420000000, -730000000);

      // Record a few randomly selected points from the features file.
      client.recordRoute(features, 10);

      // Send and receive some notes.
      CountDownLatch finishLatch = client.routeChat();

      if (!finishLatch.await(1, TimeUnit.MINUTES)) {
        client.warning("routeChat did not finish within 1 minutes");
      }
    } finally {
      channel.shutdownNow().awaitTermination(5, TimeUnit.SECONDS);
    }
  }

  private void info(String msg, Object... params) {
    logger.log(Level.INFO, msg, params);
  }

  private void warning(String msg, Object... params) {
    logger.log(Level.WARNING, msg, params);
  }

}
