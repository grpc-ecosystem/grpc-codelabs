# Copyright 2024 gRPC authors.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
"""The Python implementation of the gRPC route guide client."""

import logging

import grpc

import route_guide_pb2
import route_guide_pb2_grpc


def format_point(point):
    # Not delegating in point.__str__ because it is an empty string when its
    # values are zero. In addition, it puts a newline between the fields.
    return f"latitude: {point.latitude}, longitude: {point.longitude}"


def run():
    point = route_guide_pb2.Point(latitude=412346009, longitude=-744026814)
    channel = grpc.insecure_channel("localhost:50051")
    stub = route_guide_pb2_grpc.RouteGuideStub(channel)

    feature = stub.GetFeature(point)
    if feature.name:
        print(f"Feature called '{feature.name}' at {format_point(feature.location)}")
    else:
        print(f"Found no feature at at {format_point(feature.location)}")

    channel.close()


if __name__ == "__main__":
    logging.basicConfig()
    run()
