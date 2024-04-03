"""A module containing an implementation of Prim's algorithm for finding the minimum spanning tree of a graph."""

from heapdict import heapdict

from modgeosys.graph.types import Edge, Hop, Graph, NoNavigablePathError


def prim(graph: Graph, start_node_index: int) -> list[Edge]:
    """Implement Prim's algorithm for finding the minimum spanning tree of a graph."""

    nodes = graph.nodes
    edges = graph.edges
    adjacency_map = graph.adjacency_map()

    included_nodes = [start_node_index]
    excluded_nodes = [node_index for node_index in range(len(nodes)) if node_index != start_node_index]

    included_edges = []
    excluded_edges = tuple(edges)

    while excluded_nodes:

        # Initialize the f heapdict.
        f = heapdict()

        for candidate_edge in excluded_edges:
            if candidate_edge.node_indices[0] in included_nodes and candidate_edge.node_indices[1] in excluded_nodes:
                f[candidate_edge.weight] = candidate_edge

        # Pick the edge with the lowest f value.
        _, best_edge = f.popitem()

        # Update the included and excluded nodes and edges.
        included_nodes.extend([node_index for node_index in best_edge.node_indices if node_index not in included_nodes])
        excluded_nodes = [node_index for node_index in excluded_nodes if node_index not in included_nodes]

        included_edges.append(best_edge)
        excluded_edges = tuple(edge for edge in excluded_edges if edge != best_edge)

    return included_edges
