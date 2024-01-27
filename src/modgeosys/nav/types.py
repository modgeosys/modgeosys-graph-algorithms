"""Simple and complex data types for the navigation module."""

import bisect
from collections.abc import Callable, Mapping, Sequence
from copy import copy
from dataclasses import dataclass, field
from types import UnionType
from typing import Any

import numpy as np

type Node = tuple[int | float, int | float]
type NodeSequence = Sequence[Node]
type EdgeSequence = Sequence[Edge]
type AdjacencyMapping = Mapping[tuple[int | float, int | float], Sequence[Edge]]
type HeuristicDistanceCallable = Callable[[Node, Node], int | float]


@dataclass(order=True)
class Edge:
    """An edge in a graph."""
    weight: int | float
    node_indices: frozenset[int] = field(compare=False)

    def __post_init__(self):
        """Validate the dataclass fields."""
        validate(field_name='weight',       expected_types=(int | float),              value=self.weight)
        validate(field_name='node_indices', expected_types=frozenset,                  value=self.node_indices)

    def index_of_other_node(self, current_index: int) -> int:
        """Given one node index, return the other node index."""
        node_indices = list(self.node_indices)
        return node_indices[1] if node_indices[0] == current_index else node_indices[0]

    def __eq__(self, other):
        return self.weight == other.weight and self.node_indices == other.node_indices

    def __repr__(self):
        return f'Edge(weight={self.weight}, node_indices={self.node_indices})'

    def __hash__(self):
        return hash(self.node_indices)

    def __copy__(self):
        return Edge(weight=self.weight, node_indices=self.node_indices)

    def __deepcopy__(self, memo: Mapping | None = None):
        return Edge(weight=self.weight, node_indices=self.node_indices)



@dataclass(order=True)
class EdgeTraversal:
    """An edge traversal in a graph."""
    edge: Edge
    g: int | float | None = None
    h: int | float | None = None

    def __post_init__(self):
        """Validate the dataclass fields."""
        # validate(field_name='edge',         expected_types=Edge,                       value=self.edge)
        validate(field_name='g',            expected_types=(int | float | type(None)), value=self.g)
        validate(field_name='h',            expected_types=(int | float | type(None)), value=self.h)

    def f(self) -> int | float | None:
        """Calculate the combined cost of the edge."""
        if self.g is not None and self.h is not None:
            return self.g + self.h
        return None

    def __eq__(self, other):
        return self.edge == other.edge and self.g == other.g and self.h == other.h

    def __repr__(self):
        return f'Edge(edge={self.edge}, f={self.f()}, g={self.g}, h={self.h})'

    def __hash__(self):
        return hash(self.edge)

    def __copy__(self):
        return EdgeTraversal(edge=self.edge, g=self.g, h=self.h)

    def __deepcopy__(self, memo: Mapping | None = None):
        return EdgeTraversal(edge=self.edge, g=self.g, h=self.h)



class Graph:
    """A graph."""
    nodes: NodeSequence = field(default_factory=list)
    edges: EdgeSequence = field(default_factory=tuple)

    def __init__(self, nodes: NodeSequence, edges: EdgeSequence):
        """Initialize a graph."""
        validate(field_name='nodes', expected_types=Sequence, excluded_types=str, value=nodes)
        validate(field_name='edges', expected_types=Sequence, excluded_types=str, value=edges)

        self.nodes = [tuple(node) for node in nodes]
        self.edges = tuple(copy(edge) for edge in edges)

    def __repr__(self):
        return f'Graph(nodes={self.nodes}, edges={self.edges})'

    def __str__(self):
        return f'Graph containing these nodes: {self.nodes} and these edges: {self.edges})'

    def __eq__(self, other):
        return self.nodes == other.nodes and self.edges == other.edges

    def __hash__(self):
        return hash((self.nodes, self.edges))

    def adjacency_map(self) -> AdjacencyMapping:
        """Render an adjacency map."""

        adjacency_map = {node: [] for node in self.nodes}

        for edge in self.edges:
            for node_index in edge.node_indices:
                bisect.insort(adjacency_map[self.nodes[node_index]], edge)

        return adjacency_map

    def adjacency_matrix(self) -> np.ndarray:
        """Render an adjacency matrix."""

        adjacency_matrix = np.ones((len(self.nodes), len(self.nodes))) * np.inf

        for edge in self.edges:
            node_indices = list(edge.node_indices)
            adjacency_matrix[node_indices[0], node_indices[1]] = adjacency_matrix[node_indices[1], node_indices[0]] = edge.weight

        return adjacency_matrix


def validate(field_name: str, expected_types: type | UnionType, value: Any, excluded_types: UnionType | None = None) -> None:
    """Validate a field value."""
    if (excluded_types and isinstance(value, excluded_types)) or not isinstance(value, expected_types):
        raise NavigationFieldTypeError(field_name=field_name, expected_types=expected_types, value=value)


class NavigationFieldTypeError(TypeError):
    """Raised when an invalid type is passed to a navigation function."""
    def __init__(self, field_name: str, expected_types: type | UnionType, value: Any):
        super().__init__(f"Expected type ({expected_types}) for '{field_name}', but received type {type(value).__name__}.")


class NoNavigablePathError(Exception):
    """Raised when no path can be found to the goal node."""
    def __init__(self, start_node: Node, goal_node: Node):
        super().__init__(f'No path exists between nodes {start_node} and {goal_node}.')
