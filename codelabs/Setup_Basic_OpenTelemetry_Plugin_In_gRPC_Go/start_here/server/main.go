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
	"fmt"
	"log"
	"net"
	"time"

	"google.golang.org/grpc"

	pb "google.golang.org/grpc/examples/features/proto/echo"
)

var (
	addr               = flag.String("addr", ":50051", "the server address to connect to")
	prometheusEndpoint = flag.String("prometheus_endpoint", ":9464", "the Prometheus exporter endpoint")
)

// echoServer is used to implement pb.EchoServer.
type echoServer struct {
	pb.UnimplementedEchoServer

	addr string
}

// UnaryEcho implements pb.EchoServer UnaryEcho service. The method will return
// the message with the server address.
func (s *echoServer) UnaryEcho(ctx context.Context, req *pb.EchoRequest) (*pb.EchoResponse, error) {
	time.Sleep(2 * time.Second)
	return &pb.EchoResponse{Message: fmt.Sprintf("%s (from %s)", req.Message, s.addr)}, nil
}

func main() {
	flag.Parse()

	///////////////////////////////////////////////////////////////////////////
	// Codelab hint: Add code to setup the gRPC Go Otel plugin.
	//
	// Steps include:
	// -	Create a new prometheus exporter.
	// -	Start the Prometheus exporter.
	// -	Create a new meter provider with the prometheus exporter as server option.
	// -	Create a new gRPC server with the stats handler by passing in the server option.
	///////////////////////////////////////////////////////////////////////////

	// Create a listener on the TCP port.
	lis, err := net.Listen("tcp", *addr)
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}

	// Create a gRPC server with the stats handler.
	s := grpc.NewServer(so)
	pb.RegisterEchoServer(s, &echoServer{addr: *addr})

	log.Printf("Serving on %s\n", *addr)

	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}
