// Package main implements a RouteGuide client.
package main

import (
	"context"
	"flag"
	"log"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"

	pb "github.com/grpc-ecosystem/codelabs/getting_started_unary/routeguide"
)

var serverAddr = flag.String("addr", "localhost:50051", "The server address in the format of host:port")

func main() {
	flag.Parse()

	// Set up a connection to the gRPC server.
	conn, err := grpc.NewClient("dns:///"+*serverAddr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("fail to dial: %v", err)
	}
	defer conn.Close()

	// Create a new RouteGuide stub.
	client := pb.NewRouteGuideClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	point := &pb.Point{Latitude: 409146138, Longitude: -746188906}
	log.Printf("Getting feature for point (%d, %d)", point.Latitude, point.Longitude)

	// Call GetFeature method on the client.
	feature, err := client.GetFeature(ctx, point)
	if err != nil {
		log.Fatalf("client.GetFeature failed: %v", err)
	}
	log.Println(feature)
}
