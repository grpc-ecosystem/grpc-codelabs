# Setup Basic gRPC OpenTelemetry Plugin for gRPC-Go

# Before you Begin 

Get hands-on with gRPC's OpenTelemetry plugin for gRPC-Go in this interactive codelab! <!-- TODO(arvindbr8): Insert link once codelab is published. -->  
Designed for developers already familiar with gRPC and wanting to learn how to instrument their gRPC usage with OpenTelemetry.

## **Prerequisites** 

* [**Go**](https://golang.org/), any one of the **two latest major** [releases of Go](https://golang.org/doc/devel/release.html).  
  For installation instructions, see Go’s [Getting Started](https://golang.org/doc/install) guide.  
* [**Protocol buffer**](https://developers.google.com/protocol-buffers) **compiler**, `protoc`, [version 3](https://protobuf.dev/programming-guides/proto3).  
  For installation instructions, see [Protocol Buffer Compiler Installation](https://grpc.io/docs/protoc-installation/).  
* Use [this as a starting point](start_here/) for this codelab.

## **What you’ll learn**

* How to setup OpenTelemetry Plugin for existing gRPC Go application  
* Running a local Prometeus instance   
* Exporting metrics to Prometeus  
* View metrics from Prometeus dashboard

## **What you’ll need**

* A computer with internet connection

# Instrumenting applications with gRPC OpenTelemetry Plugin 

The client and server uses a simple gRPC HelloWorld example that we will instrument with the gRPC OpenTelemetry plugin.

## **Setup instrumentation on the client**

Open `codelabs/Setup_Basic_OpenTelemetry_Plugin_in_gRPC_Go/start_here/client/client.go` with your favorite editor

Then modify `main` to add code to setup the gRPC Go Otel plugin. 

### **Create a new Prometheus exporter**

The following snippets creates a new [prometheus Exporter](https://pkg.go.dev/go.opentelemetry.io/otel/exporters/prometheus) 

```go
exporter, err := prometheus.New()
if err != nil {
	log.Fatalf("Failed to start prometheus exporter: %v", err)
}
```

> [!NOTE]
> This codelab chooses a Prometheus Exporter is being set up on the OpenTelemetry Meter Provider.  There are other ways to export the metrics as well

### **Start the Prometheus exporter**

Spin up a new goroutine which uses `net/http` to Listen and Server at address passed in by the `prometheus_endpoint` flag

```go
go http.ListenAndServe(*prometheusEndpoint, promhttp.Handler())

```

### **Create DialOption with OpenTelemetry plugin**

Using the gRPC Otel [DialOption](https://pkg.go.dev/google.golang.org/grpc/stats/opentelemetry\#DialOption) method set the DialOption using the Promteus Meter Provider

```go
provider := metric.NewMeterProvider(metric.WithReader(exporter))
do := opentelemetry.DialOption(opentelemetry.Options{MetricsOptions: opentelemetry.MetricsOptions{MeterProvider: provider}})
```

### **Pass the DialOption when connecting to the server**

Pass `do` to gRPC.NewClient

This MeterProvider is provided to gRPC’s OpenTelemetry plugin as configuration. Once the OpenTelemetry plugin is registered globally all gRPC clients and servers will be instrumented with OpenTelemetry.

## **Setup instrumentation on the server**

Similarly, let’s add the OpenTelemetry plugin to the server as well. Open `codelabs/Setup_Basic_OpenTelemetry_Plugin_in_gRPC_Go/start_here/server/server.go` and code to setup the gRPC Go Otel plugin

### **Create a new Prometheus exporter**

```go
exporter, err := prometheus.New()
if err != nil {
	log.Fatalf("Failed to start prometheus exporter: %v", err)
}
```

### **Create a new meter provider with the prometheus exporter**

```go
provider := metric.NewMeterProvider(metric.WithReader(exporter))
go http.ListenAndServe(*prometheusEndpoint, promhttp.Handler())

so := opentelemetry.ServerOption(opentelemetry.Options{MetricsOptions: opentelemetry.MetricsOptions{MeterProvider: provider}})
```

Pass in this server option to the `gRPC.NewServer`

# Running the example and viewing metrics 

To run the server, run 

```console
$ cd server
$ go mod tidy
$ go run .
```

With a successful setup, you will see the following output for the server \- 

```console
Serving on :50051
```

While, the server is running, on another terminal, run \- 

```console
$ cd client
$ go mod tidy
$ go run .
```

A successful run will look like \- 

```console
message:"this is examples/opentelemetry (from :50051)"
message:"this is examples/opentelemetry (from :50051)"
message:"this is examples/opentelemetry (from :50051)"
```

Since we have set-up the gRPC OpenTelemetry plugin to export metrics using Prometheus. Those metrics will be available on localhost:9464 for server and localhost:9465 for client.

To see client metrics \-

```console
$ curl localhost:9465/metrics
```

The result would be of the form \- 

```console
# HELP go_gc_duration_seconds A summary of the wall-time pause (stop-the-world) duration in garbage collection cycles.
# TYPE go_gc_duration_seconds summary
go_gc_duration_seconds{quantile="0"} 0
go_gc_duration_seconds{quantile="0.25"} 0
go_gc_duration_seconds{quantile="0.5"} 0
go_gc_duration_seconds{quantile="0.75"} 0
go_gc_duration_seconds{quantile="1"} 0
go_gc_duration_seconds_sum 0
go_gc_duration_seconds_count 0
# HELP go_gc_gogc_percent Heap size target percentage configured by the user, otherwise 100. This value is set by the GOGC environment variable, and the runtime/debug.SetGCPercent function. Sourced from /gc/gogc:percent
# TYPE go_gc_gogc_percent gauge
go_gc_gogc_percent 100
# HELP go_gc_gomemlimit_bytes Go runtime memory limit configured by the user, otherwise math.MaxInt64. This value is set by the GOMEMLIMIT environment variable, and the runtime/debug.SetMemoryLimit function. Sourced from /gc/gomemlimit:bytes
# TYPE go_gc_gomemlimit_bytes gauge
go_gc_gomemlimit_bytes 9.223372036854776e+18
# HELP go_goroutines Number of goroutines that currently exist.
# TYPE go_goroutines gauge
go_goroutines 14
# HELP go_info Information about the Go environment.
# TYPE go_info gauge
go_info{version="go1.22.1"} 1
# HELP go_memstats_alloc_bytes Number of bytes allocated in heap and currently in use. Equals to /memory/classes/heap/objects:bytes.
# TYPE go_memstats_alloc_bytes gauge
go_memstats_alloc_bytes 662176
# HELP go_memstats_alloc_bytes_total Total number of bytes allocated in heap until now, even if released already. Equals to /gc/heap/allocs:bytes.
# TYPE go_memstats_alloc_bytes_total counter
go_memstats_alloc_bytes_total 662176
# HELP go_memstats_buck_hash_sys_bytes Number of bytes used by the profiling bucket hash table. Equals to /memory/classes/profiling/buckets:bytes.
# TYPE go_memstats_buck_hash_sys_bytes gauge
go_memstats_buck_hash_sys_bytes 8739
# HELP go_memstats_frees_total Total number of heap objects frees. Equals to /gc/heap/frees:objects + /gc/heap/tiny/allocs:objects.
# TYPE go_memstats_frees_total counter
go_memstats_frees_total 0
# HELP go_memstats_gc_sys_bytes Number of bytes used for garbage collection system metadata. Equals to /memory/classes/metadata/other:bytes.
# TYPE go_memstats_gc_sys_bytes gauge
go_memstats_gc_sys_bytes 1.554232e+06
# HELP go_memstats_heap_alloc_bytes Number of heap bytes allocated and currently in use, same as go_memstats_alloc_bytes. Equals to /memory/classes/heap/objects:bytes.
# TYPE go_memstats_heap_alloc_bytes gauge
go_memstats_heap_alloc_bytes 662176
# HELP go_memstats_heap_idle_bytes Number of heap bytes waiting to be used. Equals to /memory/classes/heap/released:bytes + /memory/classes/heap/free:bytes.
# TYPE go_memstats_heap_idle_bytes gauge
go_memstats_heap_idle_bytes 4.75136e+06
# HELP go_memstats_heap_inuse_bytes Number of heap bytes that are in use. Equals to /memory/classes/heap/objects:bytes + /memory/classes/heap/unused:bytes
# TYPE go_memstats_heap_inuse_bytes gauge
go_memstats_heap_inuse_bytes 2.981888e+06
# HELP go_memstats_heap_objects Number of currently allocated objects. Equals to /gc/heap/objects:objects.
# TYPE go_memstats_heap_objects gauge
go_memstats_heap_objects 4022
# HELP go_memstats_heap_released_bytes Number of heap bytes released to OS. Equals to /memory/classes/heap/released:bytes.
# TYPE go_memstats_heap_released_bytes gauge
go_memstats_heap_released_bytes 4.75136e+06
# HELP go_memstats_heap_sys_bytes Number of heap bytes obtained from system. Equals to /memory/classes/heap/objects:bytes + /memory/classes/heap/unused:bytes + /memory/classes/heap/released:bytes + /memory/classes/heap/free:bytes.
# TYPE go_memstats_heap_sys_bytes gauge
go_memstats_heap_sys_bytes 7.733248e+06
# HELP go_memstats_last_gc_time_seconds Number of seconds since 1970 of last garbage collection.
# TYPE go_memstats_last_gc_time_seconds gauge
go_memstats_last_gc_time_seconds 0
# HELP go_memstats_mallocs_total Total number of heap objects allocated, both live and gc-ed. Semantically a counter version for go_memstats_heap_objects gauge. Equals to /gc/heap/allocs:objects + /gc/heap/tiny/allocs:objects.
# TYPE go_memstats_mallocs_total counter
go_memstats_mallocs_total 4022
# HELP go_memstats_mcache_inuse_bytes Number of bytes in use by mcache structures. Equals to /memory/classes/metadata/mcache/inuse:bytes.
# TYPE go_memstats_mcache_inuse_bytes gauge
go_memstats_mcache_inuse_bytes 9600
# HELP go_memstats_mcache_sys_bytes Number of bytes used for mcache structures obtained from system. Equals to /memory/classes/metadata/mcache/inuse:bytes + /memory/classes/metadata/mcache/free:bytes.
# TYPE go_memstats_mcache_sys_bytes gauge
go_memstats_mcache_sys_bytes 15600
# HELP go_memstats_mspan_inuse_bytes Number of bytes in use by mspan structures. Equals to /memory/classes/metadata/mspan/inuse:bytes.
# TYPE go_memstats_mspan_inuse_bytes gauge
go_memstats_mspan_inuse_bytes 65440
# HELP go_memstats_mspan_sys_bytes Number of bytes used for mspan structures obtained from system. Equals to /memory/classes/metadata/mspan/inuse:bytes + /memory/classes/metadata/mspan/free:bytes.
# TYPE go_memstats_mspan_sys_bytes gauge
go_memstats_mspan_sys_bytes 81600
# HELP go_memstats_next_gc_bytes Number of heap bytes when next garbage collection will take place. Equals to /gc/heap/goal:bytes.
# TYPE go_memstats_next_gc_bytes gauge
go_memstats_next_gc_bytes 4.194304e+06
# HELP go_memstats_other_sys_bytes Number of bytes used for other system allocations. Equals to /memory/classes/other:bytes.
# TYPE go_memstats_other_sys_bytes gauge
go_memstats_other_sys_bytes 1.111813e+06
# HELP go_memstats_stack_inuse_bytes Number of bytes obtained from system for stack allocator in non-CGO environments. Equals to /memory/classes/heap/stacks:bytes.
# TYPE go_memstats_stack_inuse_bytes gauge
go_memstats_stack_inuse_bytes 655360
# HELP go_memstats_stack_sys_bytes Number of bytes obtained from system for stack allocator. Equals to /memory/classes/heap/stacks:bytes + /memory/classes/os-stacks:bytes.
# TYPE go_memstats_stack_sys_bytes gauge
go_memstats_stack_sys_bytes 655360
# HELP go_memstats_sys_bytes Number of bytes obtained from system. Equals to /memory/classes/total:byte.
# TYPE go_memstats_sys_bytes gauge
go_memstats_sys_bytes 1.1160592e+07
# HELP go_sched_gomaxprocs_threads The current runtime.GOMAXPROCS setting, or the number of operating system threads that can execute user-level Go code simultaneously. Sourced from /sched/gomaxprocs:threads
# TYPE go_sched_gomaxprocs_threads gauge
go_sched_gomaxprocs_threads 8
# HELP go_threads Number of OS threads created.
# TYPE go_threads gauge
go_threads 8
# HELP grpc_client_attempt_duration_seconds End-to-end time taken to complete a client call attempt.
# TYPE grpc_client_attempt_duration_seconds histogram
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1e-05"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="5e-05"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.0001"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.0003"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.0006"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.0008"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.001"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.002"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.003"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.004"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.005"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.006"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.008"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.01"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.013"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.016"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.02"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.025"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.03"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.04"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.05"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.065"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.08"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.1"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.13"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.16"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.2"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.25"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.3"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.4"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.5"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.65"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.8"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="2"} 0
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="5"} 26
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="10"} 26
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="20"} 26
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="50"} 26
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="100"} 26
grpc_client_attempt_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="+Inf"} 26
grpc_client_attempt_duration_seconds_sum{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev"} 52.09275054499999
grpc_client_attempt_duration_seconds_count{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev"} 26
# HELP grpc_client_attempt_rcvd_total_compressed_message_size_bytes Compressed message bytes received per call attempt.
# TYPE grpc_client_attempt_rcvd_total_compressed_message_size_bytes histogram
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0"} 0
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1024"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="2048"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="4096"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="16384"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="65536"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="262144"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1.048576e+06"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="4.194304e+06"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1.6777216e+07"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="6.7108864e+07"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="2.68435456e+08"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1.073741824e+09"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="4.294967296e+09"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="+Inf"} 26
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_sum{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev"} 1196
grpc_client_attempt_rcvd_total_compressed_message_size_bytes_count{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev"} 26
# HELP grpc_client_attempt_sent_total_compressed_message_size_bytes Compressed message bytes sent per client call attempt.
# TYPE grpc_client_attempt_sent_total_compressed_message_size_bytes histogram
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0"} 0
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1024"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="2048"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="4096"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="16384"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="65536"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="262144"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1.048576e+06"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="4.194304e+06"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1.6777216e+07"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="6.7108864e+07"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="2.68435456e+08"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1.073741824e+09"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="4.294967296e+09"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="+Inf"} 26
grpc_client_attempt_sent_total_compressed_message_size_bytes_sum{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev"} 832
grpc_client_attempt_sent_total_compressed_message_size_bytes_count{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev"} 26
# HELP grpc_client_attempt_started_total Number of client call attempts started.
# TYPE grpc_client_attempt_started_total counter
grpc_client_attempt_started_total{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev"} 27
# HELP grpc_client_call_duration_seconds Time taken by gRPC to complete an RPC from application's perspective.
# TYPE grpc_client_call_duration_seconds histogram
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1e-05"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="5e-05"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.0001"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.0003"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.0006"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.0008"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.001"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.002"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.003"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.004"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.005"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.006"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.008"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.01"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.013"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.016"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.02"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.025"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.03"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.04"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.05"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.065"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.08"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.1"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.13"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.16"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.2"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.25"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.3"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.4"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.5"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.65"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="0.8"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="1"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="2"} 0
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="5"} 26
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="10"} 26
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="20"} 26
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="50"} 26
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="100"} 26
grpc_client_call_duration_seconds_bucket{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev",le="+Inf"} 26
grpc_client_call_duration_seconds_sum{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev"} 52.35282954500001
grpc_client_call_duration_seconds_count{grpc_method="grpc.examples.echo.Echo/UnaryEcho",grpc_status="OK",grpc_target="dns:///:50051",otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev"} 26
# HELP otel_scope_info Instrumentation Scope metadata
# TYPE otel_scope_info gauge
otel_scope_info{otel_scope_name="grpc-go",otel_scope_version="1.67.0-dev"} 1
# HELP promhttp_metric_handler_requests_in_flight Current number of scrapes being served.
# TYPE promhttp_metric_handler_requests_in_flight gauge
promhttp_metric_handler_requests_in_flight 1
# HELP promhttp_metric_handler_requests_total Total number of scrapes by HTTP status code.
# TYPE promhttp_metric_handler_requests_total counter
promhttp_metric_handler_requests_total{code="200"} 1
promhttp_metric_handler_requests_total{code="500"} 0
promhttp_metric_handler_requests_total{code="503"} 0
# HELP target_info Target metadata
# TYPE target_info gauge
target_info{service_name="unknown_service:client",telemetry_sdk_language="go",telemetry_sdk_name="opentelemetry",telemetry_sdk_version="1.28.0"} 1
```

Similarly, for the server side \- 

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
$ cat > grpc_otel_go_prometheus.yml <<EOF
scrape_configs:
  - job_name: "prometheus"
    scrape_interval: 5s
    static_configs:
      - targets: ["localhost:9090"]
  - job_name: "grpc-otel-go"
    scrape_interval: 5s
    static_configs:
      - targets: ["localhost:9464", "localhost:9465"]
EOF
```

Start prometheus with the new configuration \-

```console
$ ./prometheus --config.file=grpc_otel_go_prometheus.yml
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

In the prometheus dashboards, you’ll notice that the QPS is low. See if you spot some suspicious code in the example that is limiting the QPS.

The client also sleeps for 1 second between RPCs. This can be removed as well.

For the enthusiastic, the client code limits itself to only have a single pending RPC at a given moment. This can be modified so that the client sends more RPCs without waiting for the previous ones to complete. (The solution for this has not been provided.)
