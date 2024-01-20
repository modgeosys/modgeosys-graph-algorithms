"""Simple and complex data types for the navigation module."""

import bisect
from collections.abc import Sequence, Mapping, Callable
from copy import copy
from dataclasses import dataclass, field
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
    g: int | float | None = None
    h: int | float | None = None

    def __post_init__(self):
        if not isinstance(self.weight, int):
            raise TypeError(f"The field 'weight' was assigned a value of type '{type(self.weight).__name__}', but it requires a value of type 'int'")
        if not isinstance(self.node_indices, frozenset):
            raise TypeError(f"The field 'node_indices' was assigned a value of type '{type(self.node_indices).__name__}', but it requires a value of type 'frozenset'")
        if self.g is not None and not isinstance(self.g, (int, float, type(None))):
            raise TypeError(f"The field 'g' was assigned a value of type '{type(self.g).__name__}', but it requires a value of type 'int', 'float', or 'NoneType'")
        if self.h is not None and not isinstance(self.h, (int, float, type(None))):
            raise TypeError(f"The field 'h' was assigned a value of type '{type(self.h).__name__}', but it requires a value of type 'int', 'float', or 'NoneType'")

    def f(self):
        if self.g is not None and self.h is not None:
            return self.g + self.h

    def coordinates_of_other(self, current_index: int):
        """Given one node index, return the other node index."""
        node_indices = list(self.node_indices)
        return node_indices[1] if node_indices[0] == current_index else node_indices[0]

    def __eq__(self, other):
        return self.weight == other.weight and self.node_indices == other.node_indices and self.g == other.g and self.h == other.h

    def __repr__(self):
        return f'Edge(weight={self.weight}, node_indices={self.node_indices}, f={self.f()}, g={self.g}, h={self.h})'

    def __hash__(self):
        return hash(self.node_indices)

    def __copy__(self):
        return Edge(weight=self.weight, node_indices=self.node_indices, g=self.g, h=self.h)

    def __deepcopy__(self, memo: Mapping = None):
        return Edge(weight=self.weight, node_indices=self.node_indices, g=self.g, h=self.h)


class Graph:
    """A graph."""
    nodes: NodeSequence = field(default_factory=list)
    edges: EdgeSequence = field(default_factory=tuple)

    def __init__(self, nodes: NodeSequence, edges: EdgeSequence):
        if not (isinstance(nodes, Sequence) and not isinstance(nodes, str)):
            raise TypeError(f"The field 'nodes' was assigned a value of type '{type(nodes).__name__}', but it requires a value of type 'Sequence'")
        if not (isinstance(edges, Sequence) and not isinstance(edges, str)):
            raise TypeError(f"The field 'edges' was assigned a value of type '{type(edges).__name__}', but it requires a value of type 'Sequence'")

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
            node_indices = [node_index for node_index in edge.node_indices]
            adjacency_matrix[node_indices[0], node_indices[1]] = adjacency_matrix[node_indices[1], node_indices[0]] = edge.weight

        return adjacency_matrix


class NoPathError(Exception):
    """Raised when no path can be found to the goal node."""
    pass
