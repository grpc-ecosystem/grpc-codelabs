# gRPC Java OpenTelemetry

Get hands-on with gRPC's OpenTelemetry API for Java in this interactive codelab! <!-- TODO(arvindbr8): Insert link once codelab is published. -->

Designed for developers already familiar with gRPC and wanting to learn how to instrument their gRPC usage with OpenTelemetry.

#### You'll learn how to:

- How to setup OpenTelemetry for existing gRPC Java application.
- Setup Prometheus exporter with Prometheus and export metrics to Prometheus.
- Explore collected metrics using Prometheus.

## How to use this directory

- [start_here](start_here/) directory serves as a starting point for the
  codelab.
- [complete](complete/) directory showcases the finished code.

## Before you Begin

### Prerequisites

* [JDK](https://jdk.java.net/) version 8 or higher
* Use [the current directory](https://github.com/grpc-ecosystem/grpc-codelabs/tree/main/codelabs/grpc-java-opentelemetry) as a starting point for this codelab.


## Instrumenting applications with gRPC OpenTelemetry

The client and server uses a simple gRPC HelloWorld example that we will instrument with the gRPC Java OpenTelemetry API.

## Setup instrumentation on the client

Open [`codelabs/grpc-java-opentelemetry/start_here/src/main/java/io/grpc/codelabs/opentelemetry/OpenTelemetryClient.java`](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-java-opentelemetry/start\_here/src/main/java/io/grpc/codelabs/opentelemetry/OpenTelemetryClient.java) with your favorite editor

Then modify `main` to add code to setup the gRPC Java OpenTelemetry API.

### Create Prometheus exporter

Create a PrometheusHttpServer to convert OpenTelemetry metrics to Prometheus format and  
expose these via a HttpServer. The following code snippet creates a new [Prometheus Exporter](https://javadoc.io/doc/io.opentelemetry/opentelemetry-exporter-prometheus/latest/io/opentelemetry/exporter/prometheus/PrometheusHttpServer.html).

```java
// Default prometheus port i.e `prometheusPort` has been initialized to 9465
 
PrometheusHttpServer prometheusExporter = PrometheusHttpServer.builder()
        .setPort(prometheusPort)         
        .build();
```

> [!TIP]
> This codelab uses a Prometheus Exporter to export gRPC OpenTelemetry metrics. There are other ways to export the metrics as well.


### Create OpenTelemetry SDK instance

Register above create `prometheusExporter` as MetricReader to read metrics from an [SdkMeterProvider](https://javadoc.io/doc/io.opentelemetry/opentelemetry-sdk-metrics/latest/io/opentelemetry/sdk/metrics/SdkMeterProvider.html). SdkMeterProvider is used to configure metric settings.

```java
SdkMeterProvider sdkMeterProvider = SdkMeterProvider.builder()
        .registerMetricReader(prometheusExporter)
        .build();
```

Create an instance of OpenTelemetrySdk with the above created sdkMeterProvider for SDK implementation of [OpenTelemetry](https://javadoc.io/doc/io.opentelemetry/opentelemetry-api/latest/index.html).

```java
OpenTelemetrySdk openTelemetrySdk =OpenTelemetrySdk.builder()
        .setMeterProvider(sdkMeterProvider)
        .build();
```
> [!TIP]
> This codelab uses OpenTelemetrySdk provided by OpenTelemetry. You can use your own OpenTelemetry instrumentation as well.

### Create GrpcOpenTelemetry instance

Using the GrpcOpenTelemetry [API](https://grpc.github.io/grpc-java/javadoc/io/grpc/opentelemetry/GrpcOpenTelemetry.html) set the OpenTelemetry SDK which uses Prometheus Metric exporter.

```java
GrpcOpenTelemetry grpcOpenTelmetry = GrpcOpenTelemetry.newBuilder()
        .sdk(openTelemetrySdk)
        .build();

// Registers gRPC OpenTelemetry globally.
grpcOpenTelmetry.registerGlobal();
```

Once a GrpcOpenTelemetry instance is registered globally using `registerGlobal`  all subsequently created gRPC clients and servers will be instrumented with OpenTelemetry.

### Shutdown OpenTelemetry Sdk

Shutdown needs to happen inside the ShutDownHook. `openTelemetrySdk.close()` shutdowns the SDK and also calls shutdown on the SdkMeterProvider.

```java
openTelemetrySdk.close();
```

## Setup instrumentation on the server

Similarly, let’s add the GrpcOpenTelemetry to the server as well. 
Open [`codelabs/grpc-java-opentelemetry/start_here/src/main/java/io/grpc/codelabs/opentelemetry/OpenTelemetryServer.java`](https://github.com/grpc-ecosystem/grpc-codelabs/blob/main/codelabs/grpc-java-opentelemetry/start\_here/src/main/java/io/grpc/codelabs/opentelemetry/OpenTelemetryServer.java) and add code to initiaize GrpcOpenTelemetry.

### Create a new Prometheus exporter

Since this codelab might be run from the same machine, we are using a different port to host gRPC server side metrics to avoid port conflicts while creating PrometheusHttpServer.

```java
// Default prometheus port i.e `prometheusPort` has been set to 9464

PrometheusHttpServer prometheusExporter = PrometheusHttpServer.builder()
        .setPort(prometheusPort)
        .build();
```

### Create OpenTelemetry SDK

```java
SdkMeterProvider sdkMeterProvider = SdkMeterProvider.builder()
        .registerMetricReader(prometheusExporter)
        .build();
```

### Initialize GrpcOpenTelemetry with OpenTelemetry SDK

```java
OpenTelemetrySdk openTelemetrySdk = OpenTelemetrySdk.builder()
        .setMeterProvider(sdkMeterProvider)
        .build();
```

### Shutdown OpenTelemetry Sdk

After the gRPC channel is shutdown.  Calling `openTelemetrySdk.close()` shutdowns the SDK and also calls shutdown on the SdkMeterProvider.

```java
openTelemetrySdk.close();
```


## Running the codelab and viewing metrics

Build the client and server completed code, by running
```sh
cd start_here
../gradlew installDist
```

To run the server,
```sh
./build/install/start_here/bin/opentelemetry-server
```

With a successful setup, you will see the following output for the server.

```
[date and time] io.grpc.codelabs.opentelemetry.OpenTelemetryServer start
INFO: Server started, listening on 50051
```

While, the server is running, on another terminal start client by running

```java
$ ./build/install/start_here/bin/opentelemetry-client world
```

A successful run will look like

```
[date and time]io.grpc.codelabs.opentelemetry.OpenTelemetryClient greet
INFO: Greeting: Hello world 
[date and time] io.grpc.codelabs.opentelemetry.OpenTelemetryClient greet
INFO: Will try to greet world ...
[date and time]io.grpc.codelabs.opentelemetry.OpenTelemetryClient greet
INFO: Greeting: Hello world
```

Since we have set-up gRPC OpenTelemetry to export metrics using Prometheus. 
Those metrics will be available on localhost:9464 for server and localhost:9465 for client.

To see client metrics, run

```shell
curl localhost:9465/metrics
```

The result would be of the form

```shell
# HELP grpc_client_attempt_duration_seconds Time taken to complete a client call attempt
# TYPE grpc_client_attempt_duration_seconds histogram
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.0"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1.0E-5"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="5.0E-5"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1.0E-4"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="3.0E-4"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="6.0E-4"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="8.0E-4"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.001"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.002"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.003"} 2
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.004"} 14
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.005"} 29
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.006"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.008"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.01"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.013"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.016"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.02"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.025"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.03"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.04"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.05"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.065"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.08"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.1"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.13"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.16"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.2"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.25"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.3"} 33
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.4"} 34
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.5"} 34
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.65"} 34
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.8"} 34
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1.0"} 34
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="2.0"} 34
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="5.0"} 34
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="10.0"} 34
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="20.0"} 34
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="50.0"} 34
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="100.0"} 34
grpc_client_attempt_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="+Inf"} 34
grpc_client_attempt_duration_seconds_count{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0"} 34
grpc_client_attempt_duration_seconds_sum{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0"} 0.46512665300000006
# HELP grpc_client_attempt_rcvd_total_compressed_message_size_bytes Compressed message bytes received per call attempt
# TYPE grpc_client_attempt_rcvd_total_compressed_message_size_bytes histogram
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.0"} 0
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1024.0"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="2048.0"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="4096.0"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="16384.0"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="65536.0"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="262144.0"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1048576.0"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="4194304.0"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1.6777216E7"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="6.7108864E7"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="2.68435456E8"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1.073741824E9"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="4.294967296E9"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="+Inf"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_count{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0"} 34
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_sum{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0"} 442.0
# HELP grpc_client_attempt_sent_total_compressed_message_size_bytes Compressed message bytes sent per client call attempt
# TYPE grpc_client_attempt_sent_total_compressed_message_size_bytes histogram
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.0"} 0
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1024.0"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="2048.0"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="4096.0"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="16384.0"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="65536.0"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="262144.0"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1048576.0"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="4194304.0"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1.6777216E7"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="6.7108864E7"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="2.68435456E8"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1.073741824E9"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="4.294967296E9"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="+Inf"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_count{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0"} 34
grpc_client_attempt_sent_total_compressed_message_size_bytes_sum{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0"} 238.0
# HELP grpc_client_attempt_started_total Number of client call attempts started
# TYPE grpc_client_attempt_started_total counter
grpc_client_attempt_started_total{grpc_method="helloworld.Greeter/SayHello",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0"} 34.0
# HELP grpc_client_call_duration_seconds Time taken by gRPC to complete an RPC from application's perspective
# TYPE grpc_client_call_duration_seconds histogram
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.0"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1.0E-5"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="5.0E-5"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1.0E-4"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="3.0E-4"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="6.0E-4"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="8.0E-4"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.001"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.002"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.003"} 2
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.004"} 9
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.005"} 25
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.006"} 31
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.008"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.01"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.013"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.016"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.02"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.025"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.03"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.04"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.05"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.065"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.08"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.1"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.13"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.16"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.2"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.25"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.3"} 33
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.4"} 34
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.5"} 34
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.65"} 34
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="0.8"} 34
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="1.0"} 34
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="2.0"} 34
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="5.0"} 34
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="10.0"} 34
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="20.0"} 34
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="50.0"} 34
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="100.0"} 34
grpc_client_call_duration_seconds_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0",le="+Inf"} 34
grpc_client_call_duration_seconds_count{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0"} 34
grpc_client_call_duration_seconds_sum{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-java",otel_scope_version="1.66.0"} 0.512708707
# TYPE target_info gauge
target_info{service_name="unknown_service:java",telemetry_sdk_language="java",telemetry_sdk_name="opentelemetry",telemetry_sdk_version="1.40.0"} 1
```

Similarly, for the server side metrics 

```shell
curl localhost:9464/metrics
```

## Viewing metrics on Prometheus

Here, we will setup a prometheus instance that will scrape our gRPC codelab 
client and server that are exporting metrics using prometheus.

[Download the latest release](https://prometheus.io/download) of Prometheus for your platform, then extract and run it:

```shell
tar xvfz prometheus-*.tar.gz 
cd prometheus-*
```

Create a prometheus configuration file with the following

```shell
cat > grpc_otel_java_prometheus.yml <<EOF
scrape_configs:
  - job_name: "prometheus"
    scrape_interval: 5s
    static_configs:
      - targets: ["localhost:9090"]
  - job_name: "grpc-otel-java"
    scrape_interval: 5s
    static_configs:
      - targets: ["localhost:9464", "localhost:9465"]
EOF
```

Start prometheus with the new configuration

```shell
./prometheus --config.file=grpc_otel_java_prometheus.yml
```

This will configure the metrics from the client and server codelabs processes to be scraped every 5 seconds.

Go to [http://localhost:9090/graph](http://localhost:9090/graph) to view the metrics. For example, the query:

```shell
histogram_quantile(0.5, rate(grpc_client_attempt_duration_seconds_bucket[1m])) 
```

will show a graph with the median attempt latency using 1minute as the time window for the quantile calculation.

Rate of queries:

```shell
increase(grpc_client_attempt_duration_seconds_bucket[1m])
```

## (Optional) Exercise for User 

In the prometheus dashboards, you’ll notice that the QPS is low. See if you spot some suspicious code in the example that is limiting the QPS.

The client also sleeps for 1 second between RPCs. This can be removed as well.

For the enthusiastic, the client code limits itself to only have a single pending RPC at a given moment. This can be modified so that the client sends more RPCs without waiting for the previous ones to complete. (The solution for this has not been provided.)
