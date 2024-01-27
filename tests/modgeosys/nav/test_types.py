import numpy as np
import pytest

from src.modgeosys.nav.types import Edge, Graph, NavigationFieldTypeError


def test_edge_creation_with_valid_parameters():
    edge = Edge(weight=10, node_indices=frozenset((1, 2)), g=5.0, h=5.0)
    assert edge.weight == 10.0
    assert edge.node_indices == frozenset((1, 2))
    assert edge.g == 5.0
    assert edge.h == 5.0


def test_edge_creation_with_invalid_parameters():
    with pytest.raises(NavigationFieldTypeError):
        Edge(weight="10", node_indices=frozenset((1, 2)), g=5.0, h=5.0)


def test_edge_index_of_other():
    edge = Edge(weight=10, node_indices=frozenset((1, 2)))
    assert edge.index_of_other(1) == 2
    assert edge.index_of_other(2) == 1


def test_edge_f_calculation():
    edge = Edge(weight=10.0, node_indices=frozenset((1, 2)), g=5.0, h=5.0)
    assert edge.f() == 10.0


def test_edge_f_calculation_with_none_values():
    edge = Edge(weight=10.0, node_indices=frozenset((1, 2)), g=None, h=None)
    assert edge.f() is None


def test_edge_equality():
    edge1 = Edge(weight=10.0, node_indices=frozenset((1, 2)), g=5.0, h=5.0)
    edge2 = Edge(weight=10.0, node_indices=frozenset((1, 2)), g=5.0, h=5.0)
    assert edge1 == edge2


def test_graph_creation_with_valid_parameters(valid_nodes, valid_edges1, valid_graph1):
    assert valid_graph1.nodes == valid_nodes
    assert valid_graph1.edges == valid_edges1


def test_graph_creation_with_invalid_parameters():
    with pytest.raises(NavigationFieldTypeError):
        Graph(nodes="nodes", edges="edges")


def test_graph_adjacency_mapping(valid_graph1):
    adjacency_map = valid_graph1.adjacency_map()
    assert adjacency_map == {(0.0, 0.0): [Edge(weight=1, node_indices=frozenset((0, 2)), g=None, h=None), Edge(weight=2, node_indices=frozenset((0, 1)), g=None, h=None)],
                             (0.0, 2.0): [Edge(weight=2, node_indices=frozenset((0, 1)), g=None, h=None), Edge(weight=3, node_indices=frozenset((1, 4)), g=None, h=None)],
                             (1.0, 0.0): [Edge(weight=1, node_indices=frozenset((0, 2)), g=None, h=None), Edge(weight=1, node_indices=frozenset((2, 3)), g=None, h=None)],
                             (2.0, 1.0): [Edge(weight=1, node_indices=frozenset((2, 3)), g=None, h=None), Edge(weight=1, node_indices=frozenset((3, 4)), g=None, h=None)],
                             (2.0, 3.0): [Edge(weight=1, node_indices=frozenset((3, 4)), g=None, h=None), Edge(weight=3, node_indices=frozenset((1, 4)), g=None, h=None)]}


def test_graph_adjacency_matrix(valid_graph1):
    graph = valid_graph1
    adjacency_matrix = graph.adjacency_matrix()
    assert adjacency_matrix.all() == np.array([[np.inf,    2.0,    1.0, np.inf, np.inf],
                                               [   2.0, np.inf, np.inf, np.inf,      3],
                                               [   1.0, np.inf, np.inf,    1.0, np.inf],
                                               [np.inf, np.inf,    1.0, np.inf,    1.0],
                                               [np.inf,    3.0, np.inf,    1.0, np.inf]]).all()
