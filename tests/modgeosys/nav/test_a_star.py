import pytest

from modgeosys.nav.a_star import a_star
from modgeosys.nav.distance import manhattan_distance, euclidean_distance
from modgeosys.nav.types import Edge, Graph, NoNavigablePathError


def test_a_star_finds_shortest_path_manhattan_graph1(valid_graph1):
    result = a_star(graph=valid_graph1, start_node_index=0, goal_node_index=4, heuristic_distance=manhattan_distance)

    assert len(result) == 2
    assert result == [Edge(weight=2, node_indices=frozenset({0, 1}), g=2, h=3),
                      Edge(weight=3, node_indices=frozenset({1, 4}), g=5, h=0)]


def test_a_star_finds_shortest_path_manhattan_graph2(valid_graph2):
    result = a_star(graph=valid_graph2, start_node_index=0, goal_node_index=4, heuristic_distance=manhattan_distance)

    assert len(result) == 3
    assert result == [Edge(weight=1, node_indices=frozenset({0, 2}), g=1, h=4),
                      Edge(weight=1, node_indices=frozenset({2, 3}), g=2, h=2),
                      Edge(weight=1, node_indices=frozenset({3, 4}), g=3, h=0)]


def test_a_star_with_no_path_manhattan(valid_nodes):
    with pytest.raises(NoNavigablePathError):
        a_star(graph=Graph(valid_nodes, ()), start_node_index=0, goal_node_index=3, heuristic_distance=manhattan_distance)


def test_a_star_with_single_node_path_manhattan():
    assert len(a_star(graph=Graph([(0, 0)], ()), start_node_index=0, goal_node_index=0, heuristic_distance=manhattan_distance)) == 0


# TODO: Add tests for euclidean distance, and many more permutations of the above tests.
