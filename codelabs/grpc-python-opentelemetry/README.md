# Setup Basic gRPC OpenTelemetry Plugin for gRPC-Python

## Before you Begin 

Get hands-on with gRPC's OpenTelemetry plugin for gRPC-Python in this
interactive codelab! <!-- TODO(arvindbr8): Insert link once codelab is published. -->  
Designed for developers already familiar with gRPC and wanting to learn how to
instrument their gRPC usage with OpenTelemetry.

### **Prerequisites** 

* Python 3.7 or higher  
* pip version 9.0.1 or higher

If necessary, upgrade your version of pip:

```console
$ python -m pip install --upgrade pip
```

If you cannot upgrade pip due to a system-owned installation, you can run the
example in a virtualenv:

```console
$ python -m pip install virtualenv
$ virtualenv venv
$ source venv/bin/activate
$ python -m pip install --upgrade pip
```

* Use [this as a starting point](start_here/) for this codelab.

#### Install dependencies

```console
$ cd codelabs/gRPC_Python_OpenTelemetry_Plugin/
$ python -m pip install -r requirements.txt
```

### **What you’ll learn**

* How to setup OpenTelemetry Plugin for existing gRPC Python application  
* Running a local Prometeus instance   
* Exporting metrics to Prometeus  
* View metrics from Prometeus dashboard

### **What you’ll need**

* A computer with internet connection

## Instrumenting applications with gRPC OpenTelemetry Plugin 

The client and server uses a simple gRPC HelloWorld example that we will instrument with the gRPC OpenTelemetry plugin.

### **Setup instrumentation on the client**

Open `codelabs/gRPC_Python_OpenTelemetry_Plugin/start_here/observability_greeter_client.py` with your favorite editor, first add related dependencies and macros:

```python
import logging
import time

import grpc
import grpc_observability
import helloworld_pb2
import helloworld_pb2_grpc
from opentelemetry.exporter.prometheus import PrometheusMetricReader
from opentelemetry.sdk.metrics import MeterProvider
from prometheus_client import start_http_server
```

Then transform `run()` to look like

```python
def run():
    # Start Prometheus client
    start_http_server(port=_PROMETHEUS_PORT, addr="0.0.0.0")
    # The default histogram boundaries are not granular enough for RPCs. Consider
    # override the "grpc.client.attempt.duration" view similar to csm example.
    # Link: https://github.com/grpc/grpc/blob/7407dbf21dbab125ab5eb778daab6182c009b069/examples/python/observability/csm/csm_greeter_client.py#L71C40-L71C53
    meter_provider = MeterProvider(metric_readers=[PrometheusMetricReader()])

    otel_plugin = grpc_observability.OpenTelemetryPlugin(
        meter_provider=meter_provider
    )
    otel_plugin.register_global()

    with grpc.insecure_channel(target=f"localhost:{_SERVER_PORT}") as channel:
        stub = helloworld_pb2_grpc.GreeterStub(channel)
        # Continuously send RPCs every second.
        while True:
            try:
                response = stub.SayHello(helloworld_pb2.HelloRequest(name="You"))
                print(f"Greeter client received: {response.message}")
                time.sleep(1)
            except grpc.RpcError as rpc_error:
                print("Call failed with code: ", rpc_error.code())

    # Deregister is not called in this example, but this is required to clean up.
    otel_plugin.deregister_global()
```

> [!NOTE]
> How a Prometheus Exporter is being set up on the OpenTelemetry Meter Provider. (There are other ways to export the metrics as well. This codelab chooses the prometheus exporter.) 

This MeterProvider is provided to gRPC’s OpenTelemetry plugin as configuration.
Once the OpenTelemetry plugin is registered globally all gRPC clients and
servers will be instrumented with OpenTelemetry.

### **Setup instrumentation on the server**

Similarly, let’s add the OpenTelemetry plugin to the server as well. Open `codelabs/gRPC_Python_OpenTelemetry_Plugin/start_here/observability_greeter_server.py` and change dependencies and macros to this

```python
from concurrent import futures
import logging

import grpc
import grpc_observability
import helloworld_pb2
import helloworld_pb2_grpc
from opentelemetry.sdk.metrics import MeterProvider
from opentelemetry.exporter.prometheus import PrometheusMetricReader
from prometheus_client import start_http_server

_SERVER_PORT = "50051"
_PROMETHEUS_PORT = 9464
```

Then transform `serve()` to look like this

```python
def serve():
    # Start Prometheus client
    start_http_server(port=_PROMETHEUS_PORT, addr="0.0.0.0")
    # The default histogram boundaries are not granular enough for RPCs. Consider
    # override the "grpc.client.attempt.duration" view similar to csm example.
    # Link: https://github.com/grpc/grpc/blob/7407dbf21dbab125ab5eb778daab6182c009b069/examples/python/observability/csm/csm_greeter_client.py#L71C40-L71C53
    meter_provider = MeterProvider(metric_readers=[PrometheusMetricReader()])

    otel_plugin = grpc_observability.OpenTelemetryPlugin(
        meter_provider=meter_provider
    )
    otel_plugin.register_global()

    server = grpc.server(
        thread_pool=futures.ThreadPoolExecutor(max_workers=10),
    )
    helloworld_pb2_grpc.add_GreeterServicer_to_server(Greeter(), server)
    server.add_insecure_port("[::]:" + _SERVER_PORT)
    server.start()
    print("Server started, listening on " + _SERVER_PORT)

    server.wait_for_termination()

    # Deregister is not called in this example, but this is required to clean up.
    otel_plugin.deregister_global()
```

## Running the example and viewing metrics 

To run the server, run 

```console
$ cd server
$ python -m observability_greeter_server
```

With a successful setup, you will see the following output for the server \- 

```console
Server started, listening on 50051
```

While, the server is running, on another terminal, run \- 

```console
$ cd client
$ python -m observability_greeter_client
```

A successful run will look like \- 

```console
Greeter client received: Hello You
Greeter client received: Hello You
Greeter client received: Hello You
```

Since we have set-up the gRPC OpenTelemetry plugin to export metrics using Prometheus. Those metrics will be available on localhost:9464 for server and localhost:9465 for client.

To see client metrics \-

```console
$ curl localhost:9465/metrics
```

The result would be of the form \- 

```console
# HELP python_gc_objects_collected_total Objects collected during gc
# TYPE python_gc_objects_collected_total counter
python_gc_objects_collected_total{generation="0"} 241.0
python_gc_objects_collected_total{generation="1"} 163.0
python_gc_objects_collected_total{generation="2"} 0.0
# HELP python_gc_objects_uncollectable_total Uncollectable objects found during GC
# TYPE python_gc_objects_uncollectable_total counter
python_gc_objects_uncollectable_total{generation="0"} 0.0
python_gc_objects_uncollectable_total{generation="1"} 0.0
python_gc_objects_uncollectable_total{generation="2"} 0.0
# HELP python_gc_collections_total Number of times this generation was collected
# TYPE python_gc_collections_total counter
python_gc_collections_total{generation="0"} 78.0
python_gc_collections_total{generation="1"} 7.0
python_gc_collections_total{generation="2"} 0.0
# HELP python_info Python platform information
# TYPE python_info gauge
python_info{implementation="CPython",major="3",minor="10",patchlevel="9",version="3.10.9"} 1.0
# HELP process_virtual_memory_bytes Virtual memory size in bytes.
# TYPE process_virtual_memory_bytes gauge
process_virtual_memory_bytes 1.868988416e+09
# HELP process_resident_memory_bytes Resident memory size in bytes.
# TYPE process_resident_memory_bytes gauge
process_resident_memory_bytes 4.1680896e+07
# TYPE process_resident_memory_bytes gauge                                                                                                                                                                                                                                                                21:20:16 [154/966]
process_resident_memory_bytes 4.1680896e+07
# HELP process_start_time_seconds Start time of the process since unix epoch in seconds.
# TYPE process_start_time_seconds gauge
process_start_time_seconds 1.72375679833e+09
# HELP process_cpu_seconds_total Total user and system CPU time spent in seconds.
# TYPE process_cpu_seconds_total counter
process_cpu_seconds_total 0.38
# HELP process_open_fds Number of open file descriptors.
# TYPE process_open_fds gauge
process_open_fds 9.0
# HELP process_max_fds Maximum number of open file descriptors.
# TYPE process_max_fds gauge
process_max_fds 4096.0
# HELP target_info Target metadata
# TYPE target_info gauge
target_info{service_name="unknown_service",telemetry_sdk_language="python",telemetry_sdk_name="opentelemetry",telemetry_sdk_version="1.26.0"} 1.0
# HELP grpc_client_attempt_started_total Number of client call attempts started
# TYPE grpc_client_attempt_started_total counter
grpc_client_attempt_started_total{grpc_method="other",grpc_target="localhost:50051"} 18.0
# HELP grpc_client_attempt_sent_total_compressed_message_size_bytes Compressed message bytes sent per client call attempt
# TYPE grpc_client_attempt_sent_total_compressed_message_size_bytes histogram
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="other",grpc_status="OK",grpc_target="localhost:50051",le="0.0"} 0.0
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="other",grpc_status="OK",grpc_target="localhost:50051",le="5.0"} 18.0
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="other",grpc_status="OK",grpc_target="localhost:50051",le="10.0"} 18.0
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="other",grpc_status="OK",grpc_target="localhost:50051",le="25.0"} 18.0
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="other",grpc_status="OK",grpc_target="localhost:50051",le="50.0"} 18.0
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="other",grpc_status="OK",grpc_target="localhost:50051",le="75.0"} 18.0
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="other",grpc_status="OK",grpc_target="localhost:50051",le="100.0"} 18.0
grpc_client_attempt_sent_total_compressed_message_size_bytes_bucket{grpc_method="other",grpc_status="OK",grpc_target="localhost:50051",le="250.0"} 18.0
```

Similarly, for the server side \- 

```console
$ curl localhost:9464/metrics
```

## Viewing metrics on Prometheus 

Here, we will setup a prometheus instance that will scrape our gRPC example client and server that are exporting metrics using prometheus.

[Download the latest release](https://prometheus.io/download) of Prometheus for your platform, then extract and run it:

```console
$ tar xvfz prometheus-*.tar.gz
$ cd prometheus-*
```

Create a prometheus configuration file with the following \- 

```console
$ cat > grpc_otel_python_prometheus.yml <<EOF
scrape_configs:
  - job_name: "prometheus"
    scrape_interval: 5s
    static_configs:
      - targets: ["localhost:9090"]
  - job_name: "grpc-otel-python"
    scrape_interval: 5s
    static_configs:
      - targets: ["localhost:9464", "localhost:9465"]
EOF
```

Start prometheus with the new configuration \-

```console
$ ./prometheus --config.file=grpc_otel_python_prometheus.yml
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

## (Optional) Exercise for User 

In the prometheus dashboards, you’ll notice that the QPS is low. See if you spot some suspicious code in the example that is limiting the QPS.

The client also sleeps for 1 second between RPCs. This can be removed as well.

For the enthusiastic, the client code limits itself to only have a single pending RPC at a given moment. This can be modified so that the client sends more RPCs without waiting for the previous ones to complete. (The solution for this has not been provided.)
