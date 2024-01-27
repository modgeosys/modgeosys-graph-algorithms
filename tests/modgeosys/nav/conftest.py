import pytest

from src.modgeosys.nav.types import Edge, Graph


@pytest.fixture
def valid_edge():
    return Edge(weight=10, node_indices=frozenset((1, 2)), g=5, h=5)


@pytest.fixture
def valid_nodes():
    return [(0.0, 0.0), (0.0, 2.0), (1.0, 0.0), (2.0, 1.0), (2.0, 3.0)]


@pytest.fixture
def valid_edges1():
    return (Edge(weight=2, node_indices=frozenset((0, 1))),
            Edge(weight=1, node_indices=frozenset((0, 2))),
            Edge(weight=1, node_indices=frozenset((2, 3))),
            Edge(weight=3, node_indices=frozenset((1, 4))),
            Edge(weight=1, node_indices=frozenset((3, 4))))


@pytest.fixture
def valid_edges2():
    return (Edge(weight=3, node_indices=frozenset((0, 1))),
            Edge(weight=1, node_indices=frozenset((0, 2))),
            Edge(weight=1, node_indices=frozenset((2, 3))),
            Edge(weight=3, node_indices=frozenset((1, 4))),
            Edge(weight=1, node_indices=frozenset((3, 4))))


@pytest.fixture
def valid_graph1(valid_nodes, valid_edges1):
    return Graph(valid_nodes, valid_edges1)


@pytest.fixture
def valid_graph2(valid_nodes, valid_edges2):
    return Graph(valid_nodes, valid_edges2)
