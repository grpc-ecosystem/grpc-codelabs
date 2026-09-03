/*
 * Copyright 2024 The gRPC Authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package io.grpc.codelabs.opentelemetry;

import io.grpc.Grpc;
import io.grpc.InsecureServerCredentials;
import io.grpc.Server;
import io.grpc.codelabs.helloworld.GreeterGrpc;
import io.grpc.codelabs.helloworld.HelloReply;
import io.grpc.codelabs.helloworld.HelloRequest;
import io.grpc.opentelemetry.GrpcOpenTelemetry;
import io.grpc.stub.StreamObserver;
import io.opentelemetry.exporter.prometheus.PrometheusHttpServer;
import io.opentelemetry.sdk.OpenTelemetrySdk;
import io.opentelemetry.sdk.metrics.SdkMeterProvider;
import io.opentelemetry.api.baggage.Baggage;
import io.opentelemetry.context.Context;
import java.io.IOException;
import java.util.concurrent.TimeUnit;
import java.util.logging.Logger;
import io.grpc.CallOptions;
import io.grpc.Channel;
import io.grpc.ClientCall;
import io.grpc.ClientInterceptor;
import io.grpc.ForwardingClientCall;
import io.grpc.Metadata;
import io.grpc.MethodDescriptor;
import io.opentelemetry.api.baggage.Baggage;
import io.opentelemetry.api.baggage.propagation.W3CBaggagePropagator;
import io.opentelemetry.api.trace.propagation.W3CTraceContextPropagator;
import io.opentelemetry.context.Context;
import io.opentelemetry.context.Scope;
import io.opentelemetry.context.propagation.ContextPropagators;
import io.opentelemetry.context.propagation.TextMapPropagator;
import io.opentelemetry.sdk.metrics.InstrumentSelector;
import io.opentelemetry.sdk.metrics.View;
// import io.opentelemetry.sdk.metrics.internal.view.SdkMeterProviderUtil;
import java.util.Set;
import java.util.concurrent.Executor;
import io.grpc.*;
import io.opentelemetry.api.baggage.Baggage;
import io.opentelemetry.api.baggage.propagation.W3CBaggagePropagator;
import io.opentelemetry.api.trace.propagation.W3CTraceContextPropagator;
import io.opentelemetry.context.Context;
import io.opentelemetry.context.propagation.ContextPropagators;
import io.opentelemetry.context.propagation.TextMapPropagator;
import io.opentelemetry.sdk.metrics.InstrumentSelector;
import io.opentelemetry.sdk.metrics.View;
// import io.opentelemetry.sdk.metrics.internal.view.SdkMeterProviderUtil;
import java.util.Set;

/**
 * gRPC server that manages startup/shutdown of a {@code Greeter} server and generates
 * gRPC OpenTelemetry metrics data based on the configuration.
 */
public class OpenTelemetryServer {


  private static final Logger logger = Logger.getLogger(OpenTelemetryServer.class.getName());


  private Server server;

  private void start(int port, GrpcOpenTelemetry grpcOpenTelemetry) throws IOException {

    ServerBuilder<?> builder = Grpc.newServerBuilderForPort(port, InsecureServerCredentials.create())
        .addService(new GreeterImpl());

    grpcOpenTelemetry.configureServerBuilder(builder);


    server = builder.build()
        .start();
    logger.info("Server started, listening on " + port);
  }

  private void stop() throws InterruptedException {
    if (server != null) {
      server.shutdown().awaitTermination(30, TimeUnit.SECONDS);
    }
  }

  /**
   * Await termination on the main thread since the grpc library uses daemon threads.
   */
  private void blockUntilShutdown() throws InterruptedException {
    if (server != null) {
      server.awaitTermination();
    }
  }

  /**
   * Main launches the server from the command line.
   */

  public static void main(String[] args) throws IOException, InterruptedException {
    // The port on which the server should run.
    int port = 50051;
    // The port on which prometheus metrics are exposed.
    int prometheusPort = 9464;

    if (args.length > 0) {
      if ("--help".equals(args[0])) {
        System.err.println("Usage: [port [prometheus_port]]");
        System.err.println("");
        System.err.println("  port  The port on which server will run. Defaults to " + port);
        System.err.println("  prometheusPort  The port to expose prometheus metrics. Defaults to " + prometheusPort);
        System.exit(1);
      }
      port = Integer.parseInt(args[0]);
    }
    if (args.length > 1) {
      prometheusPort = Integer.parseInt(args[1]);
    }

    ///////////////////////////////////////////////////////////////////////////
    // CODELAB SOLUTION : Register GrpcOpenTelemetry.
    ///////////////////////////////////////////////////////////////////////////

    // Adds a PrometheusHttpServer to convert OpenTelemetry metrics to Prometheus format and
    // expose these via a HttpServer exporter to the SdkMeterProvider.
    PrometheusHttpServer prometheusExporter = PrometheusHttpServer.builder()
        .setPort(prometheusPort)
        .build();
// 1. Define your baggage filter
    Set<String> allowedBaggage = Set.of("user_id", "waze_region");

// 2. Build a View that appends this baggage to gRPC metrics
    View baggageView = View.builder()
        .build();

    // Use the utility to link the baggage to this view
    // Use the utility to link the baggage to this view
/*
    SdkMeterProviderUtil.appendFilteredBaggageAttributes(
        baggageView.toBuilder(),
        allowedBaggage::contains
    );
*/

    SdkMeterProvider sdkMeterProvider = SdkMeterProvider.builder()
        .registerMetricReader(prometheusExporter)
        .registerView(
            InstrumentSelector.builder().setName("grpc.server.*").build(),
            baggageView
        )
        .build();


    ContextPropagators propagators = ContextPropagators.create(
        TextMapPropagator.composite(
            W3CTraceContextPropagator.getInstance(),
            W3CBaggagePropagator.getInstance()
        )
    );

    // Initialize OpenTelemetry SDK with MeterProvider configured with Prometheus metrics exporter
    OpenTelemetrySdk openTelemetrySdk =
        OpenTelemetrySdk.builder().setPropagators(propagators).setMeterProvider(sdkMeterProvider).build();


    // Initialize gRPC OpenTelemetry.
    // Following server metrics are enabled by default :
    //     1. grpc.server.call.started
    //     2. grpc.server.call.sent_total_compressed_message_size
    //     3. grpc.server.call.rcvd_total_compressed_message_size
    //     4. grpc.server.call.duration
    GrpcOpenTelemetry grpcOpenTelmetry = GrpcOpenTelemetry.newBuilder()
        .sdk(openTelemetrySdk)
        .build();
    // Registers gRPC OpenTelemetry globally.
    // Registers gRPC OpenTelemetry globally.
    // grpcOpenTelmetry.registerGlobal();

    final OpenTelemetryServer server = new OpenTelemetryServer();
    server.start(port, grpcOpenTelmetry);

    Runtime.getRuntime().addShutdownHook(new Thread() {
      @Override
      public void run() {
        System.err.println("*** shutting down gRPC server since JVM is shutting down");
        try {
          server.stop();
        } catch (InterruptedException e) {
          e.printStackTrace(System.err);
        }
        ///////////////////////////////////////////////////////////////////////////
        // CODELAB SOLUTION : Shutdown OpenTelemetry SDK.
        ///////////////////////////////////////////////////////////////////////////
        openTelemetrySdk.close();

        System.err.println("*** server shut down");
      }
    });

    server.blockUntilShutdown();
  }

  static class GreeterImpl extends GreeterGrpc.GreeterImplBase {

    @Override
    public void sayHello(HelloRequest req, StreamObserver<HelloReply> responseObserver) {
      HelloReply reply = HelloReply.newBuilder().setMessage("Hello " + req.getName()).build();
      responseObserver.onNext(reply);
      responseObserver.onCompleted();
    }
  }
}
