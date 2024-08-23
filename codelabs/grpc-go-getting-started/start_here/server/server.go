// Package main implements a RouteGuide server.
package main

import (
	"context"
	"encoding/json"
	"flag"
	"log"

	pb "github.com/grpc-ecosystem/codelabs/getting_started_unary/routeguide"
)

var port = flag.Int("port", 50051, "The server port")

type routeGuideServer struct {
	pb.UnimplementedRouteGuideServer

	// savedFeatures will be used to store the features. It can be initialized
	// using loadFeatures.
	//
	// Note: read-only after initialized.
	savedFeatures []*pb.Feature
}

// This is a helper function to load the features from the exampleData in
// testdata.go.
func (s *routeGuideServer) loadFeatures() {
	if err := json.Unmarshal(exampleData, &s.savedFeatures); err != nil {
		log.Fatalf("Failed to load default features: %v", err)
	}
}

func (s *routeGuideServer) GetFeature(ctx context.Context, point *pb.Point) (*pb.Feature, error) {
	///////////////////////////////////////////////////////////////////////////
	// Codelab Hint: Logic for GetFeature will be added here.
	//
	// Steps include:
	// -   Loop through the savedFeatures to find the feature that matches the
	//     point.
	// -   Return the feature if found.
	// -   Return an unnamed feature if no feature is found.
	///////////////////////////////////////////////////////////////////////////
	return nil, nil
}

func main() {
	flag.Parse()

	///////////////////////////////////////////////////////////////////////////
	// Codelab Hint: Logic for starting up a gRPC Server will be added here.
	//
	// Steps include:
	//  -   Specify the port we want to use to listen for client requests using:
	//          lis, err := net.Listen(...).
	//  -   Create an instance of the gRPC server using grpc.NewServer(...).
	//  -   Register our service implementation with the gRPC server.
	//  -   Call Serve() on the server with our port details to do a blocking
	//      wait until the process is killed or Stop() is called.
	///////////////////////////////////////////////////////////////////////////
}
