use std::error::Error;
use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::Rng;
use tokio::time;
use tonic::transport::{Channel, Endpoint};
use tonic::Request;
use protobuf::proto;

// /////////////////////////////////////////////////////////////////////////
// Codelab Hint: Bring the generated code into scope.
// /////////////////////////////////////////////////////////////////////////

use grpc_pb::route_guide_client::RouteGuideClient;
use grpc_pb::{Point, Rectangle, RouteNote};

async fn print_features(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    // --- Add logic for calling ListFeatures method on the client here. ---
	//
	// Steps include:
	// -	Call ListFeatures method on the client by passing in rect.
	// -	Loop through the features that are within the bounding Rectangle.
	// -	Print the features that are within the bounding Rectangle.

	///////////////////////////////////////////////////////////////////////////
	// Client-to-Server Streaming RPC
	//
	// Call RecordRoute method on the client.
	///////////////////////////////////////////////////////////////////////////
}

async fn run_record_route(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    // --- Add logic for calling RecordRoute method on the client here. ---
	//
	// Steps include:
	// -    Create a stream to send a sequence of points. (Hint: use rand.New() and randomPoint())
	// 		to create a new random number generator.)
	// -    Send points to the server.
	// -    Receive the response from the server.
	// -    Print the response from the server.

	///////////////////////////////////////////////////////////////////////////
	// Bidirectional Streaming RPC
	//
	// Call RouteChat method on the client.
	///////////////////////////////////////////////////////////////////////////
}

async fn run_route_chat(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    // --- Add logic for calling RouteChat method on the client here. ---
	//
	// Steps include:
	// -    Create a stream to send and receive a sequence of RouteNotes(`notes`). (Hint: client.RouteChat(ctx))
	// -    Create a goroutine which loops to receive RouteNotes from the server until the stream is closed.
	// -	In the main goroutine, send a sequence of RouteNotes to the server. Close the stream when done.
	// -	Wait for the receiving goroutine to finish. (Hint: use a channel to signal when the receiving goroutine is done.)
}

fn random_point(rng: &mut ThreadRng) -> Point {
    let latitude = (rng.random_range(0..180) - 90) * 10_000_000;
    let longitude = (rng.random_range(0..360) - 180) * 10_000_000;
    let mut point = Point::new();
    point.set_latitude(latitude);
    point.set_longitude(longitude);
    point
}

#[tokio::main]
async fn main() {
    ///////////////////////////////////////////////////////////////////////////
	// Codelab Hint: Logic for your gRPC Client will be added here.
	//
	// Steps include:
	//  -   Create a connection to the gRPC server using RouteGuideClient::new().
	//  -   Call service methods on the client to interact with the server.
	///////////////////////////////////////////////////////////////////////////
}
