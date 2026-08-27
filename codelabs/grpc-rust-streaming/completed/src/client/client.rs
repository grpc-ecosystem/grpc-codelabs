use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use grpc::client::Channel;
use grpc::credentials::LocalChannelCredentials;
use protobuf::proto;
use rand::rngs::ThreadRng;
use rand::Rng;
use tokio::time;

mod grpc_pb {
    grpc::include_generated_proto!("generated", "routeguide");
}
use grpc_pb::route_guide_client::RouteGuideClient;
use grpc_pb::{Point, Rectangle, RouteNote};

async fn print_features(client: &RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
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

    let mut stream = client.list_features(rectangle).await;

    while let Some(feature) = stream.recv().await {
        println!(
            "FEATURE: Name = \"{}\", Lat = {}, Lon = {}",
            feature.name(),
            feature.location().latitude(),
            feature.location().longitude()
        );
    }
    let status = stream.status().await;
    assert!(status.is_ok(), "{:?}", status);
    Ok(())
}

async fn run_record_route(client: &RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let point_count: i32 = rng.random_range(2..100);

    let mut points = vec![];
    for _ in 0..=point_count {
        points.push(random_point(&mut rng));
    }

    println!("Traversing {} points", points.len());
    let mut stream = client.record_route().await;

    for point in &points {
        if stream.send(point).await.is_err() {
            break;
        }
    }

    match stream.close_and_recv().await {
        Ok(response) => {
            println!(
                "SUMMARY: Feature Count = {}, Distance = {}",
                response.feature_count(),
                response.distance()
            );
        }
        Err(e) => println!("something went wrong: {e:?}"),
    }

    Ok(())
}

async fn run_route_chat(client: &RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let (mut tx, mut rx) = client.route_chat().await;

    let start = time::Instant::now();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(50));
        for _ in 0..10 {
            let time = interval.tick().await;
            let elapsed = time.duration_since(start);
            let note = proto!(RouteNote {
                location: proto!(Point {
                    latitude: 409146138 + elapsed.as_millis() as i32,
                    longitude: -746188906,
                }),
                message: format!("at {elapsed:?}"),
            });
            if tx.send(note).await.is_err() {
                return;
            }
        }
        tx.close();
    });

    while let Some(note) = rx.recv().await {
        println!(
            "Note: Latitude = {}, Longitude = {}, Message = \"{}\"",
            note.location().latitude(),
            note.location().longitude(),
            note.message()
        );
    }
    let status = rx.status().await;
    assert!(status.is_ok(), "{:?}", status);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create channel to connect to server
    let channel = Channel::builder(
        "dns:///[::1]:10000",
        Arc::new(LocalChannelCredentials::new()),
    )
    .build();

    // Create a new client
    let client = RouteGuideClient::new(channel);

    println!("\n*** SERVER STREAMING ***");
    print_features(&client).await?;

    println!("\n*** CLIENT STREAMING ***");
    run_record_route(&client).await?;

    println!("\n*** BIDIRECTIONAL STREAMING ***");
    run_route_chat(&client).await?;

    Ok(())
}

fn random_point(rng: &mut ThreadRng) -> Point {
    let latitude = (rng.random_range(0..180) - 90) * 10_000_000;
    let longitude = (rng.random_range(0..360) - 180) * 10_000_000;
    proto!(Point {
        latitude: latitude,
        longitude: longitude,
    })
}


