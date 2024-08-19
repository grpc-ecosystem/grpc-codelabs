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
