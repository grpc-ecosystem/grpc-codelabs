/*
 *
 * Copyright 2024 gRPC authors.
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
 *
 */

package main

import (
	"context"
	"flag"
	"log"
	"net/http"
	"time"

	"github.com/prometheus/client_golang/prometheus/promhttp"
	"go.opentelemetry.io/otel/exporters/prometheus"
	"go.opentelemetry.io/otel/sdk/metric"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/stats/opentelemetry"

	pb "google.golang.org/grpc/examples/features/proto/echo"
)

var (
	addr               = flag.String("addr", ":50051", "the server address to connect to")
	prometheusEndpoint = flag.String("prometheus_endpoint", ":9465", "the Prometheus exporter endpoint")
)

func main() {
	flag.Parse()

	// Create a new prometheus exporter.
	exporter, err := prometheus.New()
	if err != nil {
		log.Fatalf("Failed to start prometheus exporter: %v", err)
	}

	// Start the Prometheus exporter.
	go http.ListenAndServe(*prometheusEndpoint, promhttp.Handler())

	// Create DialOption with OpenTelemetry plugin.
	provider := metric.NewMeterProvider(metric.WithReader(exporter))
	do := opentelemetry.DialOption(opentelemetry.Options{MetricsOptions: opentelemetry.MetricsOptions{MeterProvider: provider}})

	// Set up a connection to the server.
	cc, err := grpc.NewClient(*addr, grpc.WithTransportCredentials(insecure.NewCredentials()), do)
	if err != nil {
		log.Fatalf("Failed to start NewClient: %v", err)
	}
	defer cc.Close()

	// Create a new EchoClient.
	c := pb.NewEchoClient(cc)

	// Make a RPC every second. This should trigger telemetry to be emitted from
	// the client and the server.
	ctx := context.Background()
	for {
		r, err := c.UnaryEcho(ctx, &pb.EchoRequest{Message: "this is examples/opentelemetry"})
		if err != nil {
			log.Fatalf("UnaryEcho failed: %v", err)
		}
		log.Printf("%s", r)

		// Sleep for a second.
		time.Sleep(time.Second)
	}
}
