"""A module containing an implementation of the A* algorithm for finding the shortest path between two nodes in a graph."""

from heapdict import heapdict

from modgeosys.graph.types import Edge, Hop, Graph, HeuristicDistanceCallable, NoNavigablePathError


def a_star(graph: Graph, start_node_index: int, goal_node_index: int, heuristic_distance: HeuristicDistanceCallable) -> list[Edge]:
    """Implement the A* algorithm for finding the shortest path between two nodes in a graph."""

    # Grab the nodes and adjacency map from the graph.
    nodes         = graph.nodes
    adjacency_map = graph.adjacency_map()

    # Initialize the edge hop lists.
    unhopped   = list(graph.edges)
    hopped     = []

    # Current node begins with the starting node.
    current_node_index = start_node_index

    # Initialize the f heapdict and cumulative g value.
    f = heapdict()
    g = 0

    while current_node_index != goal_node_index:

        # Calculate f for each candidate edge we could hop next.
        for candidate_edge in adjacency_map[nodes[current_node_index]]:
            if candidate_edge in unhopped:
                candidate_hop = Hop(edge=candidate_edge,
                                        g=candidate_edge.weight + g,
                                        h=heuristic_distance(nodes[candidate_edge.index_of_other_node(current_node_index)], nodes[goal_node_index]))
                f[candidate_hop.f()] = candidate_hop

        # If no path to the goal exists, raise an exception.
        if not f:
            raise NoNavigablePathError(start_node=nodes[start_node_index], goal_node=nodes[goal_node_index])

        # Pick the edge with the lowest f value.
        _, best_hop = f.popitem()

        # Update cumulative g, the index of the currently-visited node, and the edge hop lists.
        g                  = best_hop.g
        current_node_index = best_hop.edge.index_of_other_node(current_node_index)
        unhopped.remove(best_hop.edge)
        hopped.append(best_hop)

        # Clear the auto-sorted f heapdict for reuse with the next hop calculation.
        f.clear()

    return hopped
