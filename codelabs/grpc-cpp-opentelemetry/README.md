# Setup Basic OpenTelemetry Plugin in gRPC C++

## Before you Begin

Get hands-on with gRPC's OpenTelemetry plugin for C++ in this interactive
codelab! <!-- TODO(arvindbr8): Insert link once codelab is published. -->

### **Prerequisites**

* Basic understanding of gRPC and gRPC C++

* Install the following prerequisites

```console
$ sudo apt-get update -y
$ sudo apt-get upgrade -y
$ sudo apt-get install -y git curl build-essential clang
```

* This codelab will have you build examples using bazel. Install bazel using
  bazelisk. The latest version can found at https://github.com/bazelbuild/bazelisk/releases.
* A simple way to set it up is to install it as the `bazel` binary in your `PATH`.

```console
$ cp bazelisk-linux-amd64 /usr/local/bin/bazel
```

* Alternatively, you can also use CMake. Instructions for using CMake can be
  found [here](https://github.com/grpc/grpc/tree/master/src/cpp\#cmake).

### **What you’ll learn**

* How to setup OpenTelemetry Plugin for existing gRPC C++ application
* Running a local Prometheus instance
* Exporting metrics to Prometheus
* View metrics from Prometheus dashboard

### **What you’ll need**

* A computer with internet connection
* Ability to open three separate terminals.

<!-- TODO(yashkt/arvindbright) : Add some additional boilerplate stuff over here if needed. -->

## Building the example

```console
$ git clone https://github.com/grpc-ecosystem/grpc-codelabs.git
$ cd grpc-codelabs/codelabs/grpc-cpp-opentelemetry/
$ bazel build start_here/…
```

> [!NOTE]
> Building gRPC might take a few minutes. We can move on to instrumenting our gRPC example with the gRPC OpenTelemetry plugin in the meantime.

## Instrumenting applications with gRPC OpenTelemetry Plugin

The example for this codelab is in the grpc/grpc github repo at [`grpc-codelabs/codelabs/grpc-cpp-opentelemetry/`](completed/).

The client and server uses a simple gRPC HelloWorld example that we will instrument with the gRPC OpenTelemetry plugin.

Open `codelabs/grpc-cpp-opentelemetry/start_here/greeter_callback_client.cc` with your favorite editor, and transform `main()` to look like this \-

```cpp
int main(int argc, char** argv) {
 absl::ParseCommandLine(argc, argv);
 // Register a global gRPC OpenTelemetry plugin configured with a prometheus
 // exporter.
 opentelemetry::exporter::metrics::PrometheusExporterOptions opts;
 opts.url = absl::GetFlag(FLAGS_prometheus_endpoint);
 auto prometheus_exporter =
     opentelemetry::exporter::metrics::PrometheusExporterFactory::Create(opts);
 auto meter_provider =
     std::make_shared<opentelemetry::sdk::metrics::MeterProvider>();
 // The default histogram boundaries are not granular enough for RPCs. Override
 // the "grpc.client.attempt.duration" view as recommended by
 // https://github.com/grpc/proposal/blob/master/A66-otel-stats.md.
 AddLatencyView(meter_provider.get(), "grpc.client.attempt.duration", "s");
 meter_provider->AddMetricReader(std::move(prometheus_exporter));
 auto status = grpc::OpenTelemetryPluginBuilder()
                   .SetMeterProvider(std::move(meter_provider))
                   .BuildAndRegisterGlobal();
 if (!status.ok()) {
   std::cerr << "Failed to register gRPC OpenTelemetry Plugin: "
             << status.ToString() << std::endl;
   return static_cast<int>(status.code());
 }


 // Continuously send RPCs.
 RunClient(absl::GetFlag(FLAGS_target));


 return 0;
}
```

> [!NOTE]
> A Prometheus Exporter is being set up on the OpenTelemetry Meter Provider.
> (There are other ways to export the metrics as well. This codelab chooses the
> prometheus exporter.) This MeterProvider is provided to gRPC’s OpenTelemetry
> plugin as configuration. Once the OpenTelemetry plugin is registered globally
> all gRPC clients and servers will be instrumented with OpenTelemetry.

Similarly, let’s add the OpenTelemetry plugin to the server as well. Open `codelabs/grpc-cpp-opentelemetry/start_here/greeter_callback_server.cc` and transform main to look like this

```cpp
int main(int argc, char** argv) {
  absl::ParseCommandLine(argc, argv);
  // Register a global gRPC OpenTelemetry plugin configured with a prometheus
  // exporter.
  opentelemetry::exporter::metrics::PrometheusExporterOptions opts;
  opts.url = absl::GetFlag(FLAGS_prometheus_endpoint);
  auto prometheus_exporter =
      opentelemetry::exporter::metrics::PrometheusExporterFactory::Create(opts);
  auto meter_provider =
      std::make_shared<opentelemetry::sdk::metrics::MeterProvider>();
  // The default histogram boundaries are not granular enough for RPCs. Override
  // the "grpc.server.call.duration" view as recommended by
  // https://github.com/grpc/proposal/blob/master/A66-otel-stats.md.
  AddLatencyView(meter_provider.get(), "grpc.server.call.duration", "s");
  meter_provider->AddMetricReader(std::move(prometheus_exporter));
  auto status = grpc::OpenTelemetryPluginBuilder()
                    .SetMeterProvider(std::move(meter_provider))
                    .BuildAndRegisterGlobal();
  if (!status.ok()) {
    std::cerr << "Failed to register gRPC OpenTelemetry Plugin: "
              << status.ToString() << std::endl;
    return static_cast<int>(status.code());
  }
  RunServer(absl::GetFlag(FLAGS_port));
  return 0;
}
```

> [!NOTE]
> The required header files and build dependencies have already been added for convenience.

```cpp
#include "opentelemetry/exporters/prometheus/exporter_factory.h"
#include "opentelemetry/exporters/prometheus/exporter_options.h"
#include "opentelemetry/sdk/metrics/meter_provider.h"

#include <grpcpp/ext/otel_plugin.h>
```

Build dependencies added in `BUILD` file \-

```bazel
cc_binary(
   name = "greeter_callback_client",
   srcs = ["greeter_callback_client.cc"],
   defines = ["BAZEL_BUILD"],
   deps = [
       "//util:util",
       "@com_github_grpc_grpc//:grpc++",
       "@com_github_grpc_grpc//:grpcpp_otel_plugin",
       "@com_google_absl//absl/flags:flag",
       "@com_google_absl//absl/flags:parse",
       "@io_opentelemetry_cpp//exporters/prometheus:prometheus_exporter",
       "@io_opentelemetry_cpp//sdk/src/metrics",
   ],
)
```

# Running the example and viewing metrics

To run the server, run \-

```console
$ bazel run start_here:greeter_callback_server
```

With a successful setup, you will see the following output for the server \-

```console
Server listening on 0.0.0.0:50051
```

While, the server is running, on another terminal, run the client \-

```console
$ bazel run start_here:greeter_callback_client
```

A successful run will look like \-

```console
Greeter received: Hello world
Greeter received: Hello world
Greeter received: Hello world
Greeter received: Hello world
Greeter received: Hello world
Greeter received: Hello world
Greeter received: Hello world
Greeter received: Hello world
Greeter received: Hello world
Greeter received: Hello world
Greeter received: Hello world
```

Since we have set-up the gRPC OpenTelemetry plugin to export metrics using Prometheus. Those metrics will be available on localhost:9464 for server and localhost:9465 for client.

To see client metrics \-

```console
$ curl localhost:9465/metrics
```

The result would be of the form \-

```console
# HELP exposer_transferred_bytes_total Transferred bytes to metrics services
# TYPE exposer_transferred_bytes_total counter
exposer_transferred_bytes_total 0
# HELP exposer_scrapes_total Number of times metrics were scraped
# TYPE exposer_scrapes_total counter
exposer_scrapes_total 0
# HELP exposer_request_latencies Latencies of serving scrape requests, in microseconds
# TYPE exposer_request_latencies summary
exposer_request_latencies_count 0
exposer_request_latencies_sum 0
exposer_request_latencies{quantile="0.5"} Nan
exposer_request_latencies{quantile="0.9"} Nan
exposer_request_latencies{quantile="0.99"} Nan
# HELP target Target metadata
# TYPE target gauge
target_info{otel_scope_name="grpc-c++",otel_scope_version="1.67.0-dev",service_name="unknown_service",telemetry_sdk_version="1.13.0",telemetry_sdk_name="opentelemetry",telemetry_sdk_language="cpp"} 1 1721958543107
# HELP grpc_client_attempt_rcvd_total_compressed_message_size_bytes Compressed message bytes received per call attempt
# TYPE grpc_client_attempt_rcvd_total_compressed_message_size_bytes histogram
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_count{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-c++",otel_scope_version="1.67.0-dev"} 96 1721958543107
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_sum{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-c++",otel_scope_version="1.67.0-dev"} 1248 1721958543107
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-c++",otel_scope_version="1.67.0-dev",le="0"} 0 1721958543107
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-c++",otel_scope_version="1.67.0-dev",le="5"} 0 1721958543107
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-c++",otel_scope_version="1.67.0-dev",le="10"} 0 1721958543107
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-c++",otel_scope_version="1.67.0-dev",le="25"} 96 1721958543107
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-c++",otel_scope_version="1.67.0-dev",le="50"} 96 1721958543107
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="helloworld.Greeter/SayHello",grpc_status="OK",grpc_target="dns:///localhost:50051",otel_scope_name="grpc-c++",otel_scope_version="1.67.0-dev",le="75"} 96 1721958543107
```

Similarly, for the server side metrics \-

```console
$ curl localhost:9464/metrics
```

# Viewing metrics on Prometheus

Here, we will setup a prometheus instance that will scrape our gRPC example client and server that are exporting metrics using prometheus.

[Download the latest release](https://prometheus.io/download) of Prometheus for your platform, then extract and run it:

```console
$ tar xvfz prometheus-*.tar.gz
$ cd prometheus-*
```

Create a prometheus configuration file with the following \-

```console
$ cat > grpc_otel_cpp_prometheus.yml <<EOF
scrape_configs:
  - job_name: "prometheus"
    scrape_interval: 5s
    static_configs:
      - targets: ["localhost:9090"]
  - job_name: "grpc-otel-cpp"
    scrape_interval: 5s
    static_configs:
      - targets: ["localhost:9464", "localhost:9465"]
EOF
```

Start prometheus with the new configuration \-

```console
$ ./prometheus --config.file=grpc_otel_cpp_prometheus.yml
```

This will configure the metrics from the client and server codelab processes to be scraped every 5 seconds.

Go to [http://localhost:9090/graph](http://localhost:9090/graph) to view the metrics. For example, the query \-

```
histogram_quantile(0.5, rate(grpc_client_attempt_duration_seconds_bucket[1m]))
```

will show a graph with the median attempt latency using 1minute as the time window for the quantile calculation.

Rate of queries \-

```
increase(grpc_client_attempt_duration_seconds_bucket[1m])
```

# (Optional) Exercise for User

In the prometheus dashboards, you’ll notice that the QPS is low. See if you can identify some suspicious code in the example that is limiting the QPS.

For the enthusiastic, the client code limits itself to only have a single pending RPC at a given moment. This can be modified so that the client sends more RPCs without waiting for the previous ones to complete. (The solution for this has not been provided.)
