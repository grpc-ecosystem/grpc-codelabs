use tonic::transport::Server;
use tonic::{Request, Response, Status};
use serde::Deserialize;
use std::fs::File;
use protobuf::proto;

// /////////////////////////////////////////////////////////////////////////
// Codelab Hint: Bring the generated code into scope.
// /////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct RouteGuideService {
    ///////////////////////////////////////////////////////////////////////////
    // Codelab Hint: Define the RouteGuideService struct.
    ///////////////////////////////////////////////////////////////////////////
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        // /////////////////////////////////////////////////////////////////////////
        // Codelab Hint: Logic for GetFeature will be added here.
        //
        // Steps include:
        // -   Loop through server's features to find the feature that matches the
        //     point.
        // -   Return the feature if found.
        // -   Return an unnamed feature if no feature is found.
        // /////////////////////////////////////////////////////////////////////////
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///////////////////////////////////////////////////////////////////////////
	// Codelab Hint: Logic for starting up a gRPC Server will be added here.
	//
	// Steps include:
	//  -   Specify the port we want to use to listen for client requests using
	//  -   Create an instance of the gRPC server using RouteGuideServer::new().
	//  -   Register our service implementation with the server.
	///////////////////////////////////////////////////////////////////////////
}

#[derive(Debug, Deserialize)]
struct JsonFeature {
    location: JsonPoint,
    name: String,
}

#[derive(Debug, Deserialize)]
struct JsonPoint {
    latitude: i32,
    longitude: i32,
}

#[allow(dead_code)]
pub fn load() -> Vec<Feature> {
    let data_dir = std::path::PathBuf::from_iter([
        std::env!("CARGO_MANIFEST_DIR"),
        "src",                           
        "data"                           
    ]);
    let file = File::open(data_dir.join("route_guide_db.json")).expect("failed to open data file");
    let decoded: Vec<JsonFeature> =
        serde_json::from_reader(&file).expect("failed to deserialize features");
    decoded
        .into_iter()
        .map(|feature| proto!(Feature {
            name: feature.name,
            location: proto!(Point {
                longitude: feature.location.longitude,
                latitude: feature.location.latitude,
            }),
        }))
        .collect()
}