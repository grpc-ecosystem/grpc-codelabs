// Package main implements a RouteGuide server.
package main

import (
	"flag"
	"math/rand"

	pb "github.com/grpc-ecosystem/codelabs/getting_started_streaming/routeguide"
)

var serverAddr = flag.String("addr", "localhost:50051", "The server address in the format of host:port")

func main() {
	flag.Parse()

	///////////////////////////////////////////////////////////////////////////
	// Codelab Hint: Logic for your gRPC Client will be added here.
	//
	// Steps include:
	//  -   Create a connection to the gRPC server using grpc.NewClient(...).
	//          conn, err := grpc.NewClient(...)
	//  -   Create a new RouteGuide stub.
	///////////////////////////////////////////////////////////////////////////

	///////////////////////////////////////////////////////////////////////////
	// Server-to-Client Streaming RPC
	//
	// Call ListFeatures method on the client.
	///////////////////////////////////////////////////////////////////////////

	// --- Add logic for calling ListFeatures method on the client here. ---
	//
	// Steps include:
	// -	Create a Rectangle with the some coordinates.
	// -	Call ListFeatures method on the client.
	// -	Loop through the features that are within the bounding Rectangle.
	// -	Print the features that are within the bounding Rectangle.

	///////////////////////////////////////////////////////////////////////////
	// Client-to-Server Streaming RPC
	//
	// Call RecordRoute method on the client.
	///////////////////////////////////////////////////////////////////////////

	// --- Add logic for calling RecordRoute method on the client here. ---
	//
	// Steps include:
	//
	//
	// -    Create a stream to send a sequence of points. (Hint: use rand.New() and randomPoint())
	// 		to create a new random number generator.)
	// -    Send a sequence of points to the server.
	// -    Receive the response from the server.
	// -    Print the response from the server.

	///////////////////////////////////////////////////////////////////////////
	// Bidirectional Streaming RPC
	//
	// Call RouteChat method on the client.
	///////////////////////////////////////////////////////////////////////////

	// --- Add logic for calling RouteChat method on the client here. ---
	//
	// Steps include:
	// -    Create a stream to send and receive a sequence of RouteNotes. (Hint: client.RouteChat(ctx))
	// -    Create a goroutine which loops to receive RouteNotes from the server until the stream is closed.
	// -	In the main goroutine, send a sequence of RouteNotes to the server. Close the stream when done.
	// -	Wait for the receiving goroutine to finish. (Hint: use a channel to signal when the receiving goroutine is done.)
}

func randomPoint(r *rand.Rand) *pb.Point {
	lat := (r.Int31n(180) - 90) * 1e7
	long := (r.Int31n(360) - 180) * 1e7
	return &pb.Point{Latitude: lat, Longitude: long}
}
