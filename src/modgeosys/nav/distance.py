"""Heuristic distance functions. """

import math

from modgeosys.nav.types import Node


def manhattan_distance(a: Node, b: Node) -> int | float:
    """Calculate the Manhattan distance between two points. """
    return abs(a[0] - b[0]) + abs(a[1] - b[1])

def euclidean_distance(a: Node, b: Node) -> int | float:
    """Calculate the Euclidean distance between two points. """
    return math.hypot(a[0] - b[0], a[1] - b[1])