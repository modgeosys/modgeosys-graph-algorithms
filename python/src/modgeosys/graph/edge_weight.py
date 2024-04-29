"""Edge weight functions."""

from modgeosys.graph.types import Edge, Graph


def length_cost_per_unit(graph: Graph, edge: Edge) -> float:
    cost_per_unit = edge.properties['cost_per_unit']
    heuristic_distance = graph.distance_function
    attached_nodes = [graph.nodes[node_index] for node_index in edge.node_indices]
    return cost_per_unit * heuristic_distance(attached_nodes[0], attached_nodes[1])
