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
import time

import grpc

import route_guide_pb2
import route_guide_pb2_grpc
import route_guide_resources


def format_point(point):
    return f"(lat={point.latitude}, lng={point.longitude})"


def make_route_note(message, point):
    return route_guide_pb2.RouteNote(message=message, location=point)


def generate_route(feature_list, count=10):
    for _ in range(0, count):
        random_feature = random.choice(feature_list)
        print(f"Visiting point {format_point(random_feature.location)}")
        yield random_feature.location


def guide_list_features(stub):
    _lo = route_guide_pb2.Point(latitude=400000000, longitude=-750000000)
    _hi = route_guide_pb2.Point(latitude=420000000, longitude=-730000000)
    rectangle = route_guide_pb2.Rectangle(
        lo=_lo,
        hi=_hi,
    )
    print("Looking for features between 40, -75 and 42, -73")

    features = stub.ListFeatures(rectangle)
    for feature in features:
        print(
            f"Feature called '{feature.name}'"
            f" at {format_point(feature.location)}"
        )


def guide_record_route(stub):
    feature_list = route_guide_resources.read_route_guide_database()
    route_iterator = generate_route(feature_list)

    route_summary = stub.RecordRoute(route_iterator)
    print(f"Finished trip with {route_summary.point_count} points")
    print(f"Passed {route_summary.feature_count} features")
    print(f"Traveled {route_summary.distance} meters")
    print(f"It took {route_summary.elapsed_time} seconds")


def generate_notes():
    home = route_guide_pb2.Point(latitude=1, longitude=1)
    work = route_guide_pb2.Point(latitude=2, longitude=2)
    notes = [
        make_route_note("Departing from home", home),
        make_route_note("Arrived at work", work),
        make_route_note("Having lunch at work", work),
        make_route_note("Departing from work", work),
        make_route_note("Arrived home", home),
    ]
    for note in notes:
        print(
            f"Sending RouteNote for {format_point(note.location)}:"
            f" {note.message}"
        )
        yield note
        # Sleep to simulate moving from one point to another.
        # Only for demonstrating the order of the messages.
        time.sleep(0.1)


def guide_route_chat(stub):
    responses = stub.RouteChat(generate_notes())
    for response in responses:
        print(
            "< Found previous note at"
            f" {format_point(response.location)}: {response.message}"
        )


def run():
    with grpc.insecure_channel("localhost:50051") as channel:
        stub = route_guide_pb2_grpc.RouteGuideStub(channel)
        print("-------------- ListFeatures --------------")
        guide_list_features(stub)
        print("-------------- RecordRoute --------------")
        guide_record_route(stub)
        print("-------------- RouteChat --------------")
        guide_route_chat(stub)


if __name__ == "__main__":
    logging.basicConfig()
    run()
