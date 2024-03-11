"""Heuristic distance functions."""

import numpy as np

from modgeosys.nav.types import Node


def manhattan_distance(a: Node, b: Node) -> int | float:
    """Calculate the Manhattan distance between two points."""
    return np.sum(np.abs(a - b))


def euclidean_distance(a: Node, b: Node) -> int | float:
    """Calculate the Euclidean distance between two points."""
    diff = (a - b).coordinates
    return np.hypot(*diff)


def least_squares_distance(a: Node, b: Node) -> int | float:
    """Calculate the least squares distance between two points."""
    diff = (a - b).coordinates
    return np.sum(np.square(diff))
