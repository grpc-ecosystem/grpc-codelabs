
package io.grpc.examples.routeguide;

import io.grpc.Grpc;
import io.grpc.InsecureServerCredentials;
import io.grpc.Server;
import io.grpc.ServerBuilder;
import java.io.IOException;
import java.net.URL;
import java.util.concurrent.TimeUnit;
import java.util.logging.Logger;

/**
 * A sample gRPC server that serve the RouteGuide (see route_guide.proto) service.
 */
public class RouteGuideServer {
  private static final Logger logger = Logger.getLogger(RouteGuideServer.class.getName());

  private final int port;
  private final Server server;

  public RouteGuideServer(int port) throws IOException {
    this(port, RouteGuideUtil.getDefaultFeaturesFile());
  }

  /** Create a RouteGuide server listening on {@code port} using {@code featureFile} database. */
  public RouteGuideServer(int port, URL featureFile) throws IOException {
    this(Grpc.newServerBuilderForPort(port, InsecureServerCredentials.create()),
        port, RouteGuideUtil.parseFeatures(featureFile));
  }

  /** Create a RouteGuide server using serverBuilder as a base and features as data. */
  public RouteGuideServer(ServerBuilder<?> serverBuilder, int port, Collection<Feature> features) {
    this.port = port;
    /****************************************************************
     * Codelab Hint: Create a server object using the passed in serverBuilder
     *
    server =
     ****************************************************************/
  }

  /** Start serving requests. */
  public void start() throws IOException {

  /****************************************************************
   * Codelab Hint: Start the server
   ****************************************************************/

    logger.info("Server started, listening on " + port);

// Always configure a shutdown hook so that you can cleanly stop your server
    Runtime.getRuntime().addShutdownHook(new Thread() {
      @Override
      public void run() {
        // Use stderr here since the logger may have been reset by its JVM shutdown hook.
        System.err.println("*** shutting down gRPC server since JVM is shutting down");
        try {
          RouteGuideServer.this.stop();
        } catch (InterruptedException e) {
          e.printStackTrace(System.err);
        }
        System.err.println("*** server shut down");
      }
    });
  }

  /** Stop serving requests and shutdown resources. */
  public void stop() throws InterruptedException {
    if (server != null) {
      server.shutdown().awaitTermination(30, TimeUnit.SECONDS);
    }
  }

  /**
   * Await termination on the main thread since the grpc library uses daemon threads.
   */
  private void blockUntilShutdown() throws InterruptedException {
    if (server != null) {
  /****************************************************************
   * Codelab Hint: wait for server termination
   ****************************************************************/
    }
  }

  /**
   * Main method.
   */
  public static void main(String[] args) throws Exception {
    RouteGuideServer server = new RouteGuideServer(8980);
    server.start();
    server.blockUntilShutdown();
  }

  /**
   * Our implementation of RouteGuide service.
   *
   * <p>See route_guide.proto for details of the methods.
   */
  private static class RouteGuideService extends RouteGuideGrpc.RouteGuideImplBase {
    private final Collection<Feature> features;
    private final ConcurrentMap<Point, List<RouteNote>> routeNotes =
        new ConcurrentHashMap<Point, List<RouteNote>>();

    RouteGuideService(Collection<Feature> features) {
      this.features = features;
    }

    /**
     * Gets the {@link Feature} at the requested {@link Point}. If no feature at that location
     * exists, an unnamed feature is returned at the provided location.
     *
     * @param request the specified location for the feature.
     * @param responseObserver the observer that will receive the feature at the requested point.
     */
    @Override
    public void getFeature(Point request, StreamObserver<Feature> responseObserver) {
  /****************************************************************
   * Codelab Hint: use checkFeature so send an appropriate response
   * and mark the RPC complete
   ****************************************************************/

    }

    /**
     * Gets all features contained within the given bounding {@link Rectangle}.
     *
     * @param request the bounding rectangle for the requested features.
     * @param responseObserver the observer that will receive the features.
     */
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
          /***************************************************************
           * Codelab Hint: send this feature to the client using responseObserver
           ***************************************************************/
        }
      }
      /***************************************************************
       * Codelab Hint: inform client that rpc is complete using responseObserver
       ***************************************************************/
    }

    /**
     * Gets a stream of points, and responds with statistics about the "trip": number of points,
     * number of known features visited, total distance traveled, and total time spent.
     *
     * @param responseObserver an observer to receive the response summary.
     * @return an observer to receive the requested route points.
     */
    @Override
    public StreamObserver<Point> recordRoute(final StreamObserver<RouteSummary> responseObserver) {
      return new StreamObserver<Point>() {
        int pointCount;
        int featureCount;
        int distance;
        Point previous;
        final long startTime = System.nanoTime();

        @Override
        public void onNext(Point point) {
          pointCount++;
          if (RouteGuideUtil.exists(checkFeature(point))) {
            featureCount++;
          }
          // For each point after the first, add the incremental distance from the previous point to
          // the total distance value.
          if (previous != null) {
            distance += calcDistance(previous, point);
          }
          previous = point;
        }

        @Override
        public void onError(Throwable t) {
          logger.log(Level.WARNING, "recordRoute cancelled");
        }

        @Override
        public void onCompleted() {
          long seconds = NANOSECONDS.toSeconds(System.nanoTime() - startTime);
          responseObserver.onNext(RouteSummary.newBuilder().setPointCount(pointCount)
              .setFeatureCount(featureCount).setDistance(distance)
              .setElapsedTime((int) seconds).build());
          /***************************************************************
           * Codelab Hint: inform client that rpc is complete using responseObserver
           ***************************************************************/
        }
      };
    }

    /**
     * Receives a stream of message/location pairs, and responds with a stream of all previous
     * messages at each of those locations.
     *
     * @param responseObserver an observer to receive the stream of previous messages.
     * @return an observer to handle requested message/location pairs.
     */
    @Override
    public StreamObserver<RouteNote> routeChat(final StreamObserver<RouteNote> responseObserver) {
      return new StreamObserver<RouteNote>() {
        @Override
        public void onNext(RouteNote note) {
          List<RouteNote> notes = getOrCreateNotes(note.getLocation());

          // Respond with all previous notes at this location.
          for (RouteNote prevNote : notes.toArray(new RouteNote[0])) {
            /***************************************************************
             * Codelab Hint: send prevNote to the client using responseObserver
             ***************************************************************/
          }

          // Now add the new note to the list
          notes.add(note);
        }

        @Override
        public void onError(Throwable t) {
          logger.log(Level.WARNING, "routeChat cancelled");
        }

        @Override
        public void onCompleted() {
          /***************************************************************
           * Codelab Hint: inform client that rpc is complete using responseObserver
           ***************************************************************/
        }
      };
    }

    /**
     * Get the notes list for the given location. If missing, create it.
     */
    private List<RouteNote> getOrCreateNotes(Point location) {
      List<RouteNote> notes = Collections.synchronizedList(new ArrayList<>());
      List<RouteNote> prevNotes = routeNotes.putIfAbsent(location, notes);
      return prevNotes != null ? prevNotes : notes;
    }

    /**
     * Gets the feature at the given point.
     *
     * @param location the location to check.
     * @return The feature object at the point. Note that an empty name indicates no feature.
     */
    private Feature checkFeature(Point location) {
      for (Feature feature : features) {
        if (feature.getLocation().getLatitude() == location.getLatitude()
            && feature.getLocation().getLongitude() == location.getLongitude()) {
          return feature;
        }
      }

      // No feature was found, return an unnamed feature.
      return Feature.newBuilder().setName("").setLocation(location).build();
    }

  }
}

