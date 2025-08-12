// use serde::Deserialize;
// use std::fs::File;
// use protobuf::proto;

// mod grpc_pb {
//     include!(concat!(
//         env!("CARGO_MANIFEST_DIR"),
//         "/generated/generated.rs"
//     ));
//     include!(concat!(
//         env!("CARGO_MANIFEST_DIR"),
//         "/generated/routeguide_grpc.pb.rs"
//     ));
// }

// #[derive(Debug, Deserialize)]
// struct JsonFeature {
//     location: Location,
//     name: String,
// }

// #[derive(Debug, Deserialize)]
// struct Location {
//     latitude: i32,
//     longitude: i32,
// }

// pub use grpc_pb::{
//     route_guide_server::{RouteGuideServer, RouteGuide},
//     Point, Feature, Rectangle, RouteNote, RouteSummary
// };

// #[allow(dead_code)]
// pub fn load() -> Vec<Feature> {
//     let data_dir = std::path::PathBuf::from_iter([
//         std::env!("CARGO_MANIFEST_DIR"),
//         "src",                           
//         "data"                           
//     ]);
//     let file = File::open(data_dir.join("route_guide_db.json")).expect("failed to open data file");
//     let decoded: Vec<JsonFeature> =
//         serde_json::from_reader(&file).expect("failed to deserialize features");
//     decoded
//         .into_iter()
//         .map(|feature| proto!(Feature {
//             name: feature.name,
//             location: proto!(Point {
//                 longitude: feature.location.longitude,
//                 latitude: feature.location.latitude,
//             }),
//         }))
//         .collect()
// }
