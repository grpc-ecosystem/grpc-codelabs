use std::error::Error;
use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::Rng;
use tokio::time;
use tonic::transport::{Channel, Endpoint};
use tonic::Request;
use protobuf::proto;

mod grpc_pb {
    // Include message code.
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/generated.rs"
    ));
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/generated/routeguide_grpc.pb.rs"
    ));
}

use grpc_pb::route_guide_client::RouteGuideClient;
use grpc_pb::{Point, Rectangle, RouteNote};


async fn print_features(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let rectangle = proto!(Rectangle {
        lo: proto!(Point {
            latitude: 400_000_000,
            longitude: -750_000_000,
        }),
        hi: proto!(Point {
            latitude: 420_000_000,
            longitude: -730_000_000,
        }),
    });

    let mut stream = client
        .list_features(Request::new(rectangle))
        .await?
        .into_inner();

    while let Some(feature) = stream.message().await? {
        println!("FEATURE: Name = \"{}\", Lat = {}, Lon = {}",
            feature.name(),
            feature.location().latitude(),
            feature.location().longitude());
        }
    Ok(())
}

async fn run_record_route(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let point_count: i32 = rng.random_range(2..100);

    let mut points = vec![];
    for _ in 0..=point_count {
        points.push(random_point(&mut rng))
    }

    println!("Traversing {} points", points.len());
    let request = Request::new(tokio_stream::iter(points));

    match client.record_route(request).await {
        Ok(response) => {
            let response = response.into_inner();
            println!("SUMMARY: Feature Count = {}, Distance = {}", response.feature_count(), response.distance())},
        Err(e) => println!("something went wrong: {e:?}"),
    }

    Ok(())
}

async fn run_route_chat(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let start = time::Instant::now();
    let outbound = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            let time = interval.tick().await;
            let elapsed = time.duration_since(start);
            let note = proto!(RouteNote {
                location: proto!(Point {
                    latitude: 409146138 + elapsed.as_secs() as i32,
                    longitude: -746188906,
                }),
                message: format!("at {elapsed:?}"),
            });
            yield note;
        }
    };
    let response = client.route_chat(Request::new(outbound)).await?;
    let mut inbound = response.into_inner();
    while let Some(note) = inbound.message().await? {
        println!("Note: Latitude = {}, Longitude = {}, Message = \"{}\"",
            note.location().latitude(),
            note.location().longitude(),
            note.message());
        }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Create endpoint to connect to
    let endpoint = Endpoint::new("http://[::1]:10000")?; 
    let channel = endpoint.connect().await?;             

    // Create a new client
    let mut client = RouteGuideClient::new(channel);

    println!("\n*** SERVER STREAMING ***");
    print_features(&mut client).await?;

    println!("\n*** CLIENT STREAMING ***");
    run_record_route(&mut client).await?;

    println!("\n*** BIDIRECTIONAL STREAMING ***");
    run_route_chat(&mut client).await?;

    Ok(())
}

fn random_point(rng: &mut ThreadRng) -> Point {
    let latitude = (rng.random_range(0..180) - 90) * 10_000_000;
    let longitude = (rng.random_range(0..360) - 180) * 10_000_000;
    proto!(Point {
        latitude: latitude,
        longitude: longitude
    })
}

