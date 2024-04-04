"""A module containing an implementation of Prim's algorithm for finding the minimum spanning tree of a graph."""

from modgeosys.graph.types import Edge, Graph, ValidEdgeCallable, NoNavigablePathError
from modgeosys.graph.edge_validation import edge_is_always_valid


def prim(graph: Graph, start_node_index: int, edge_is_valid: ValidEdgeCallable = edge_is_always_valid) -> set[Edge]:
    """Implement Prim's algorithm for finding the minimum spanning tree of a graph."""

    nodes = graph.nodes
    edges = graph.edges

    included_node_indices = {start_node_index}
    excluded_node_indices = (set(range(len(nodes))))
    excluded_node_indices.remove(start_node_index)

    included_edges = set()
    excluded_edges = sorted(edges, key=lambda edge: edge.weight)

    next_included_node_index_1 = start_node_index
    next_included_node_index_2 = None

    while excluded_node_indices:

        best_edge = None

        # Find the candidate edge with the lowest weight that passes the validity test.
        while excluded_edges and not best_edge:

            candidate_edge = find_first_edge_with_node_index(excluded_edges, next_included_node_index_1, next_included_node_index_2)

            if candidate_edge and edge_is_valid(candidate_edge):

                best_edge = candidate_edge
                new_node_index = best_edge.index_of_other_node(best_edge.node_indices - included_node_indices)

                if new_node_index not in included_node_indices:
                    included_node_indices.add(new_node_index)
                if new_node_index in excluded_node_indices:
                    excluded_node_indices.remove(new_node_index)

                included_edges.add(best_edge)
                excluded_edges.remove(best_edge)

                next_included_node_index_2 = next_included_node_index_1
                next_included_node_index_1 = new_node_index

                break

        if not best_edge:
            raise NoNavigablePathError(start_node=nodes[start_node_index])

    return included_edges


def find_first_edge_with_node_index(edges: list[tuple[Edge, int]], node_index_1: int, node_index_2: int = None) -> tuple[Edge, int]:
    """Perform a sequential search for the first edge with the given node index."""
    for edge in edges:
        if node_index_1 in edge.node_indices or (node_index_2 and node_index_2 in edge.node_indices):
            return edge
    return None