use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use serde::Deserialize;
use std::fs::File;
use protobuf::proto;

// /////////////////////////////////////////////////////////////////////////
// Codelab Hint: Bring the generated code into scope.
// /////////////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize)]
struct JsonFeature {
    location: Location,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: i32,
    longitude: i32,
}

pub use grpc_pb::{
    route_guide_server::{RouteGuideServer, RouteGuide},
    Point, Feature, Rectangle, RouteNote, RouteSummary
};

#[derive(Debug)]
pub struct RouteGuideService {
    features: Arc<Vec<Feature>>,
}
type ListFeaturesStream = Pin<Box<dyn Stream<Item = Result<Feature, Status>> + Send + 'static>>;
type RouteChatStream = Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send + 'static>>;

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {

    async fn list_features(
        &self,
        request: Request<Rectangle>,
    ) -> Result<Response<ListFeaturesStream>, Status> {
        ///////////////////////////////////////////////////////////////////////////
        // Codelab Hint: Logic for ListFeature will be added here.
        //
        // Steps include:
        // -    Loop through the features to find the features that are within
        //      the given bounding Rectangle.
        // -    Send the features that are within the bounding Rectangle to the
        //		client.
        // -    Return an error if there is an issue sending the feature.
        ///////////////////////////////////////////////////////////////////////////
    }

    async fn record_route(
        &self,
        request: Request<tonic::Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        // /////////////////////////////////////////////////////////////////////////
        // Codelab Hint: Logic for RecordRoute will be added here.
        // Steps include:
        // -    Loop until the end of the stream
        // -    Calculate the distance between the last point and the current point.
        // -    Update the point_count, feature_count, and distance.
        // -    Calculate the total time spent.
        // -    Send the RouteSummary to the client.
        // /////////////////////////////////////////////////////////////////////////
    }

    async fn route_chat(
        &self,
        request: Request<tonic::Streaming<RouteNote>>,
    ) -> Result<Response<RouteChatStream>, Status> {
        ///////////////////////////////////////////////////////////////////////////
        // Codelab Hint: Logic for RouteChat will be added here.
        //
        // Steps include:
        // -    Loop until the end of the stream.
        // -    Append the message to the notes map. 
        // -    Send all previous messages at each of those locations to the client.
        // -    Return the stream.
        ///////////////////////////////////////////////////////////////////////////
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///////////////////////////////////////////////////////////////////////////
	// Codelab Hint: Logic for starting up a gRPC Server will be added here.
	//
	// Steps include:
	//  -   Specify the port we want to use to listen for client requests 
	//  -   Create an instance of the gRPC server using RouteGuideServer.new(...).
	//  -   Register our service implementation with the gRPC server.
	//  -   Call serve() on the server with our port details to do a blocking
	//      wait until the process is killed or Stop() is called.
	///////////////////////////////////////////////////////////////////////////
}

fn in_range(point: &Point, rect: &Rectangle) -> bool {
    use std::cmp;

    let lo = rect.lo();
    let hi = rect.hi();

    let left = cmp::min(lo.longitude(), hi.longitude());
    let right = cmp::max(lo.longitude(), hi.longitude());
    let top = cmp::max(lo.latitude(), hi.latitude());
    let bottom = cmp::min(lo.latitude(), hi.latitude());

    point.longitude() >= left
        && point.longitude() <= right
        && point.latitude() >= bottom
        && point.latitude() <= top
}

/// Calculates the distance between two points using the "haversine" formula.
/// This code was taken from http://www.movable-type.co.uk/scripts/latlong.html.
fn calc_distance(p1: &Point, p2: &Point) -> i32 {
    const CORD_FACTOR: f64 = 1e7;
    const R: f64 = 6_371_000.0; // meters

    let lat1 = p1.latitude() as f64 / CORD_FACTOR;
    let lat2 = p2.latitude() as f64 / CORD_FACTOR;
    let lng1 = p1.longitude() as f64 / CORD_FACTOR;
    let lng2 = p2.longitude() as f64 / CORD_FACTOR;

    let lat_rad1 = lat1.to_radians();
    let lat_rad2 = lat2.to_radians();

    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lng = (lng2 - lng1).to_radians();

    let a = (delta_lat / 2f64).sin() * (delta_lat / 2f64).sin()
        + (lat_rad1).cos() * (lat_rad2).cos() * (delta_lng / 2f64).sin() * (delta_lng / 2f64).sin();

    let c = 2f64 * a.sqrt().atan2((1f64 - a).sqrt());

    (R * c) as i32
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