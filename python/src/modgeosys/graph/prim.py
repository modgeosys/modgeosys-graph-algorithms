"""A module containing an implementation of Prim's algorithm for finding the minimum spanning tree of a graph."""

import itertools

from modgeosys.graph.types import Edge, Graph, ValidEdgeCallable, NoNavigablePathError
from modgeosys.graph.edge_validation import edge_is_always_valid


def prim(graph: Graph, start_node_index: int, edge_is_valid: ValidEdgeCallable = edge_is_always_valid) -> list[Edge]:
    """Implement Prim's algorithm for finding the minimum spanning tree of a graph."""

    nodes = graph.nodes
    excluded_edges_adjacency_map = graph.adjacency_map()

    included_node_indices = [start_node_index]
    excluded_node_indices = (list(range(len(nodes))))
    excluded_node_indices.remove(start_node_index)

    included_edges = []

    while excluded_node_indices:

        next_included_node_index = included_node_indices[-1]
        next_included_node = nodes[next_included_node_index]
        best_edge = None

        # Find the candidate edge with the lowest weight that passes validity test.
        for i, candidate_edge in enumerate(excluded_edges_adjacency_map[next_included_node]):
            if edge_is_valid(candidate_edge):
                best_edge = candidate_edge
                included_node_indices.append(best_edge.index_of_other_node(next_included_node_index))
                excluded_node_indices.remove(best_edge.index_of_other_node(next_included_node_index))
                included_edges.append(best_edge)
            excluded_edges_adjacency_map[next_included_node][i] = None

        if not best_edge:
            raise NoNavigablePathError(start_node=nodes[start_node_index])

        # Remove all edges from the node entry in the adjacency map that are set to None.
        excluded_edges_adjacency_map[next_included_node] = list(itertools.compress(excluded_edges_adjacency_map[next_included_node], excluded_edges_adjacency_map[next_included_node]))
        # If the resulting node list is empty, remove the node from the adjacency map.
        if not excluded_edges_adjacency_map[next_included_node]:
            del excluded_edges_adjacency_map[next_included_node]

    return included_edges
