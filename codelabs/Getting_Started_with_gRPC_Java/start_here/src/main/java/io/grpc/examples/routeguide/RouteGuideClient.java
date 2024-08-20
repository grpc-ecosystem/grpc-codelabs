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

  /** Construct client stub for accessing RouteGuide server using the existing channel. */
  public RouteGuideClient(Channel channel) {
    /****************************************************************
     * Codelab Hint: Create a blocking stub for your service
     * using the code generated from the proto file (RouteGuideGrpc)
     *
     blockingStub =
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
      feature = blockingStub.getFeature(request);
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
   * Phase 2 of the code lab
   * Blocking server-streaming example. Calls listFeatures with a rectangle of interest. Prints each
   * response feature as it arrives.
   * Requires using the async API.
   */
  public void listFeatures(int lowLat, int lowLon, int hiLat, int hiLon) {
    info("*** ListFeatures: lowLat={0} lowLon={1} hiLat={2} hiLon={3}", lowLat, lowLon, hiLat,
        hiLon);

        /****************************************************************
         * Codelab Hint: Implement this method as phase 2 of the code lab
         *
         Rectangle request =
         ****************************************************************/
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

      // Looking for a valid feature
      client.getFeature(409146138, -746188906);

      // Feature missing.
      client.getFeature(0, 0);

/********** PHASE 2
      // Looking for features between 40, -75 and 42, -73.
      client.listFeatures(400000000, -750000000, 420000000, -730000000);

      // Record a few randomly selected points from the features file.
      client.recordRoute(features, 10);
******************/
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
