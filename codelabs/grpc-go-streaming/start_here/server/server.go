// Package main implements a RouteGuide server.
package main

import (
	"encoding/json"
	"flag"
	"log"
	"sync"

	pb "github.com/grpc-ecosystem/codelabs/getting_started_streaming/routeguide"
)

var port = flag.Int("port", 50051, "The server port")

type routeGuideServer struct {
	pb.UnimplementedRouteGuideServer

	// savedFeatures will be used to store the features. It can be initialized
	// using loadFeatures.
	//
	// Note: read-only after initialized.
	savedFeatures []*pb.Feature

	mu         sync.Mutex // protects routeNotes
	routeNotes map[string][]*pb.RouteNote
}

// This is a helper function to load the features from the exampleData in
// testdata.go.
func (s *routeGuideServer) loadFeatures() {
	if err := json.Unmarshal(exampleData, &s.savedFeatures); err != nil {
		log.Fatalf("Failed to load default features: %v", err)
	}
}

// ListFeatures lists all features contained within the given bounding Rectangle.
func (s *routeGuideServer) ListFeatures(rect *pb.Rectangle, stream pb.RouteGuide_ListFeaturesServer) error {
	///////////////////////////////////////////////////////////////////////////
	// Codelab Hint: Logic for ListFeature will be added here.
	//
	// Steps include:
	// -    Loop through the savedFeatures to find the features that are within
	//      the given bounding Rectangle.
	// -    Send the features that are within the bounding Rectangle to the
	//		client.
	// -    Return an error if there is an issue sending the feature.
	///////////////////////////////////////////////////////////////////////////
	return nil
}

// RecordRoute records a route composited of a sequence of points.
//
// It gets a stream of points, and responds with statistics about the "trip":
// number of points,  number of known features visited, total distance traveled, and
// total time spent.
func (s *routeGuideServer) RecordRoute(stream pb.RouteGuide_RecordRouteServer) error {
	///////////////////////////////////////////////////////////////////////////
	// Codelab Hint: Logic for RecordRoute will be added here.
	//
	// Steps include:
	// -    Loop until the end of the stream, i.e., until io.EOF is received.
	// -    Calculate the distance between the last point and the current point.
	// -    Update the pointCount, featureCount, and distance.
	// -    Calculate the total time spent.
	// -    Send the RouteSummary to the client.
	// -    Return an error if there is an issue sending the RouteSummary.
	///////////////////////////////////////////////////////////////////////////
	return nil
}

// RouteChat receives a stream of message/location pairs, and responds with a stream of all
// previous messages at each of those locations.
func (s *routeGuideServer) RouteChat(stream pb.RouteGuide_RouteChatServer) error {
	///////////////////////////////////////////////////////////////////////////
	// Codelab Hint: Logic for RouteChat will be added here.
	//
	// Steps include:
	// -    Loop until the end of the stream, i.e., until io.EOF is received.
	// -    Serialize the location to a string. (Hint: use the serialize function)
	// -    Append the message to the routeNotes map. (Hint: use s.routeNotes, after acquiring the lock)
	// -    Send all previous messages at each of those locations to the client.
	// -    Return an error if there is an issue sending the RouteNote.
	///////////////////////////////////////////////////////////////////////////
	return nil
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
