"""A module containing an implementation of the A* algorithm for finding the shortest path between two nodes in a graph."""

from heapdict import heapdict

from modgeosys.nav.distance import manhattan_distance
from modgeosys.nav.types import Edge, EdgeTransit, Graph, HeuristicDistanceCallable, NoNavigablePathError


def a_star(graph: Graph, start_node_index: int, goal_node_index: int, heuristic_distance: HeuristicDistanceCallable) -> list[Edge]:
    """Implement the A* algorithm for finding the shortest path between two nodes in a graph."""

    # Grab the nodes and adjacency map from the graph.
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

    while current_node_index != goal_node_index:

        # Calculate f for each candidate edge we could traverse next.
        for candidate_edge in adjacency_map[nodes[current_node_index]]:
            if candidate_edge in untraversed:
                candidate_transit = EdgeTransit(edge=candidate_edge,
                                                g=candidate_edge.weight + g,
                                                h=heuristic_distance(nodes[candidate_edge.index_of_other_node(current_node_index)], nodes[goal_node_index]))
                f[candidate_transit.f()] = candidate_transit

        # If no path to the goal exists, raise an exception.
        if not f:
            raise NoNavigablePathError(start_node=nodes[start_node_index], goal_node=nodes[goal_node_index])

        # Pick the edge with the lowest f value.
        _, best_transit = f.popitem()

        # Update cumulative g, the index of the currently-visited node, and the edge traversal lists.
        g = best_transit.g
        current_node_index = best_transit.edge.index_of_other_node(current_node_index)
        untraversed.remove(best_transit.edge)
        traversed.append(best_transit)

        # Clear the auto-sorted f heapdict for reuse with the next traversal calculation.
        f.clear()

    return traversed
