// Package main implements a RouteGuide client.
package main

import (
	"flag"
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
	//  -   Call service methods on the client to interact with the server.
	///////////////////////////////////////////////////////////////////////////
}
