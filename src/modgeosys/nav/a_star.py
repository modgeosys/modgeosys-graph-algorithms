from heapdict import heapdict

from modgeosys.nav.types import Edge, Graph, EdgeSequence, HeuristicDistanceCallable
from modgeosys.nav.distance import manhattan_distance


def a_star(graph: Graph, start_node_index: int, end_node_index: int, heuristic_distance: HeuristicDistanceCallable = manhattan_distance) -> list[Edge]:
    """Implementation of the A* algorithm for finding the shortest path between two nodes in a graph."""

    # Grab the nodes and adjacency map.
    nodes = graph.nodes
    adjacency_map = graph.adjacency_map()

    # Initialize the edge traversal lists.
    untraversed = list(graph.edges)
    traversed = []

    # Current node begins with the starting node.
    current_node_index = start_node_index

    # Initialize the f heapdict and cumulative g value.
    f = heapdict()
    g = 0

    while current_node_index != end_node_index:

        # Calculate f for each candidate edge we could traverse next.
        for candidate_edge in adjacency_map[nodes[current_node_index]]:
            if candidate_edge in untraversed:
                candidate_edge.g = candidate_edge.weight + g
                candidate_edge.h = heuristic_distance(nodes[candidate_edge.coordinates_of_other(current_node_index)], nodes[end_node_index])
                f[candidate_edge.f()] = candidate_edge

        # Pick the edge with the lowest f value.
        best_f, best_transit_edge = f.popitem()

        # Update cumulative g, edge traversal lists, and the index of the currently-visited node.
        g = best_transit_edge.g
        untraversed.remove(best_transit_edge)
        traversed.append(best_transit_edge)
        current_node_index = best_transit_edge.coordinates_of_other(current_node_index)

        # Clear the auto-sorted f heapdict for reuse with the next traversal calculation.
        f.clear()

    return traversed
