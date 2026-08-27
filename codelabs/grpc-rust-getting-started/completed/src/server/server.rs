use tonic::transport::Server;
use tonic::{Request, Response, Status};
use serde::Deserialize;
use std::fs::File;
use protobuf::proto;

mod grpc_pb {
    grpc::include_generated_proto!("generated", "routeguide");
}

pub use grpc_pb::{
    route_guide_server::{RouteGuideServer, RouteGuide},
    Point, Feature,
};

#[derive(Debug)]
pub struct RouteGuideService {
    features: Vec<Feature>,
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, request: Request<Point>) -> Result<Response<Feature>, Status> {
        println!("GetFeature = {:?}", request);
        let requested_point = request.get_ref();
        for feature in self.features.iter() {
            if feature.location().latitude() == requested_point.latitude() {
                if feature.location().longitude() == requested_point.longitude(){
                    return Ok(Response::new(feature.clone()))
                };
            };    
        }
        Ok(Response::new(Feature::default()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();
    println!("RouteGuideServer listening on: {addr}");
    let route_guide = RouteGuideService {
        features: load(),
    };
    let svc = RouteGuideServer::new(route_guide);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
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