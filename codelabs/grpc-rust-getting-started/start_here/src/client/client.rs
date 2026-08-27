use std::sync::Arc;
use grpc::client::Channel;
use grpc::credentials::LocalChannelCredentials;
use protobuf::proto;

// /////////////////////////////////////////////////////////////////////////
// Codelab Hint: Bring the generated code into scope.
// /////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///////////////////////////////////////////////////////////////////////////
    // Codelab Hint: Logic for your gRPC Client will be added here.
    //
    // Steps include:
    //  -   Create a Channel using Channel::builder() and LocalChannelCredentials.
    //  -   Create a RouteGuideClient instance.
    //  -   Create a Point message and call get_feature().
    ///////////////////////////////////////////////////////////////////////////

    Ok(()) // Hint: Replace with channel creation, client initialization, and get_feature call.
}