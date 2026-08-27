use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use grpc::client::Channel;
use grpc::credentials::LocalChannelCredentials;
use protobuf::proto;
use rand::rngs::ThreadRng;
use rand::Rng;
use tokio::time;

// /////////////////////////////////////////////////////////////////////////
// Codelab Hint: Bring the generated code into scope.
// /////////////////////////////////////////////////////////////////////////

async fn print_features(client: &RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    // --- Add logic for calling ListFeatures method on the client here. ---
    //
    // Steps include:
    // -    Call list_features method on the client by passing in rectangle.
    // -    Loop through features using stream.recv().await until None.
    // -    Print each received feature and check stream.status().await.

    Ok(()) // Hint: Replace with call to list_features method on the client.
}

async fn run_record_route(client: &RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    // --- Add logic for calling RecordRoute method on the client here. ---
    //
    // Steps include:
    // -    Start the RPC using client.record_route().await.
    // -    Send points to the server using stream.send(&point).await.
    // -    Close and receive the response summary using stream.close_and_recv().await.
    // -    Print the response summary from the server.

    Ok(()) // Hint: Replace with call to record_route method on the client.
}

async fn run_route_chat(client: &RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    // --- Add logic for calling RouteChat method on the client here. ---
    //
    // Steps include:
    // -    Start the RPC using client.route_chat().await to get (tx, rx).
    // -    Spawn an asynchronous task with tokio::spawn to send RouteNotes using tx.send(note).await, then tx.close().
    // -    In the current task, loop to receive RouteNotes from rx.recv().await until None.
    // -    Check rx.status().await.

    Ok(()) // Hint: Replace with call to route_chat method on the client.
}

fn random_point(rng: &mut ThreadRng) -> Point {
    let latitude = (rng.random_range(0..180) - 90) * 10_000_000;
    let longitude = (rng.random_range(0..360) - 180) * 10_000_000;
    proto!(Point {
        latitude: latitude,
        longitude: longitude,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///////////////////////////////////////////////////////////////////////////
    // Codelab Hint: Logic for your gRPC Client will be added here.
    //
    // Steps include:
    //  -   Create a Channel using Channel::builder() and LocalChannelCredentials.
    //  -   Create a RouteGuideClient instance.
    //  -   Call print_features, run_record_route, and run_route_chat.
    ///////////////////////////////////////////////////////////////////////////

    Ok(()) // Hint: Replace with client initialization and method calls.
}

