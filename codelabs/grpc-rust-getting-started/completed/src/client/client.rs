use std::sync::Arc;
use grpc::client::Channel;
use grpc::credentials::LocalChannelCredentials;
use protobuf::proto;

mod grpc_pb {
    grpc::include_generated_proto!("generated", "routeguide");
}

use grpc_pb::{
    route_guide_client::RouteGuideClient,
    Point,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new gRPC channel
    let channel = Channel::builder(
        "dns:///[::1]:10000",
        Arc::new(LocalChannelCredentials::new()),
    )
    .build();

    // Create a new client
    let client = RouteGuideClient::new(channel);

    println!("*** SIMPLE RPC ***");
    let point = proto!(Point {
        latitude: 409_146_138,
        longitude: -746_188_906,
    });
    let response = client
        .get_feature(point)
        .await
        .expect("RPC error");

    println!("Response = Name = \"{}\", Latitude = {}, Longitude = {}",
        response.name(),
        response.location().latitude(),
        response.location().longitude());
    Ok(())
}

