#[path="routeguide.u.pb.rs"]
#[allow(nonstandard_style, unused, unreachable_pub)]
#[doc(hidden)]
mod internal_do_not_use_routeguide;

#[allow(nonstandard_style, unused)]
#[doc(inline)]
pub use internal_do_not_use_routeguide::*;
#[allow(nonstandard_style, unused)]
pub mod __unstable {
pub static ROUTEGUIDE_DESCRIPTOR_INFO: ::protobuf::__internal::runtime::__unstable::DescriptorInfo = ::protobuf::__internal::runtime::__unstable::DescriptorInfo {
  descriptor: b"\n\x10routeguide.proto\x12\nrouteguide\",\n\x05Point\x12\x10\n\x08latitude\x18\x01 \x01(\x05\x12\x11\n\tlongitude\x18\x02 \x01(\x05\"<\n\x07\x46\x65\x61ture\x12\x0c\n\x04name\x18\x01 \x01(\t\x12#\n\x08location\x18\x02 \x01(\x0b\x32\x11.routeguide.Point2D\n\nRouteGuide\x12\x36\n\nGetFeature\x12\x11.routeguide.Point\x1a\x13.routeguide.Feature\"\x00\x42\x30\n\x1bio.grpc.examples.routeguideB\x0fRouteGuideProtoP\x01\x62\x06proto3",
  deps: &[
  ],
};
}
