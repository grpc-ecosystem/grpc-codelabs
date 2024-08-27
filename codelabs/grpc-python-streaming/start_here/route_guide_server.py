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
"""The Python implementation of the gRPC route guide server."""

import logging
import math
import time
from concurrent import futures

import grpc

import route_guide_pb2
import route_guide_pb2_grpc
import route_guide_resources


def get_feature(feature_db, point):
    """Returns Feature at given location or None."""
    for feature in feature_db:
        if feature.location == point:
            return feature
    return None


def get_distance(start, end):
    """Distance between two points."""
    coord_factor = 10000000.0
    lat_1 = start.latitude / coord_factor
    lat_2 = end.latitude / coord_factor
    lon_1 = start.longitude / coord_factor
    lon_2 = end.longitude / coord_factor
    lat_rad_1 = math.radians(lat_1)
    lat_rad_2 = math.radians(lat_2)
    delta_lat_rad = math.radians(lat_2 - lat_1)
    delta_lon_rad = math.radians(lon_2 - lon_1)

    # Formula is based on http://mathforum.org/library/drmath/view/51879.html
    a = pow(math.sin(delta_lat_rad / 2), 2) + (
        math.cos(lat_rad_1) * math.cos(lat_rad_2) * pow(math.sin(delta_lon_rad / 2), 2)
    )
    c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))
    R = 6371000
    # metres
    return R * c


class RouteGuideServicer(route_guide_pb2_grpc.RouteGuideServicer):
    """Provides methods that implement functionality of route guide server."""

    def __init__(self):
        self.db = route_guide_resources.read_route_guide_database()

    def ListFeatures(self, request, context):
        """
        Codelab Hint: implement ListFeatures() here.
        Steps include:
          1. Loop through the savedFeatures in self.db to find the features that
        are within the given bounding Rectangle.
          2. Send the features that are within the bounding Rectangle to the
        client.
        """

    def RecordRoute(self, request_iterator, context):
        """
        Codelab Hint: implement RecordRoute() here.
        Steps include:
          1. Loop until the end of the stream (request_iterator).
          2. Calculate the distance between the last point and the current point
        using get_distance.
          3. Update the pointCount, featureCount, and distance.
          4. Calculate the total time spent.
          5. Send the RouteSummary to the client.
        """

    def RouteChat(self, request_iterator, context):
        """
        Codelab Hint: implement RouteChat() here.
        Steps include:
          1. Loop until the end of the stream (request_iterator).
          2. Send all previous messages at each of those locations to the client.
        """


def serve():
    """
    Codelab Hint: Logic for starting up a gRPC Server will be added here.
    Steps include:
     1. create gRPC server using grpc.server().
     2. register RouteGuideServicer to the server.
     3. start the server.
    """


if __name__ == "__main__":
    logging.basicConfig()
    serve()
