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
import random

import grpc

import route_guide_pb2
import route_guide_pb2_grpc
import route_guide_resources


def make_route_note(message, latitude, longitude):
    """Codelab Hint: Call route_guide_pb2.RouteNote here."""


def format_point(point):
    # Not delegating in point.__str__ because it is an empty string when its
    # values are zero. In addition, it puts a newline between the fields.
    return f"latitude: {point.latitude}, longitude: {point.longitude}"


def guide_list_features(stub):
    """
    Codelab Hint: Implement server streaming RPC ListFeatures here.
    Steps include:
      1: Create route_guide_pb2.Point and call route_guide_pb2.Rectangle. latitude
    and longitude for points should be 400000000, 750000000 and 420000000, 730000000 respectively.
      2. Call stub.ListFeatures and list features in rectangle.
    """


def generate_route(feature_list):
    for _ in range(0, 10):
        random_feature = random.choice(feature_list)
        print(f"Visiting point {format_point(random_feature.location)}")
        yield random_feature.location


def guide_record_route(stub):
    feature_list = route_guide_resources.read_route_guide_database()
    route_iterator = generate_route(feature_list)
    # Codelab Hint: Call RecordRoute with route_iterator here.


def generate_messages():
    messages = [
        make_route_note("First message", 0, 0),
        make_route_note("Second message", 0, 1),
        make_route_note("Third message", 1, 0),
        make_route_note("Fourth message", 0, 0),
        make_route_note("Fifth message", 1, 0),
    ]
    for msg in messages:
        print(f"Sending {msg.message} at {format_point(msg.location)}")
        yield msg


def guide_route_chat(stub):
    """Codelab Hint: Call RouteChat with generate_messages() here."""


def run():
    """
    Codelab Hint: Logic for your gRPC Client will be added here.
    Steps include:
     1. Implement make_route_note, guide_list_features, guide_record_route and guide_route_chat.
     2. Create a connection to the gRPC server using grpc.insecure_channel().
     3. Call service methods on the client to interact with the server.
    """


if __name__ == "__main__":
    logging.basicConfig()
    run()
