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
	"fmt"
	"log"
	"net"

	"google.golang.org/grpc"
	"google.golang.org/protobuf/proto"

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
	for _, feature := range s.savedFeatures {
		if proto.Equal(feature.Location, point) {
			return feature, nil
		}
	}
	// No feature was found, return an unnamed feature
	return &pb.Feature{Location: point}, nil
}

func main() {
	flag.Parse()
	lis, err := net.Listen("tcp", fmt.Sprintf("localhost:%d", *port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	var opts []grpc.ServerOption
	grpcServer := grpc.NewServer(opts...)

	s := &routeGuideServer{}
	s.loadFeatures()
	pb.RegisterRouteGuideServer(grpcServer, s)
	grpcServer.Serve(lis)
}
