"""Usage example(s)."""

import pickle
from pprint import pprint

from modgeosys.graph.edge_weight import length_cost_per_unit
from modgeosys.graph.distance import manhattan_distance, euclidean_distance
from modgeosys.graph.types import Graph, COMPUTED_WEIGHT

from modgeosys.graph.a_star import a_star
from modgeosys.graph.prim import prim

# Define a toy graph.
toy_graph = Graph.from_edge_definitions(edge_definitions=((((0.0, 0.0), (0.0, 2.0)), COMPUTED_WEIGHT, {'cost_per_unit': 2}),
                                                          (((0.0, 0.0), (1.0, 0.0)), COMPUTED_WEIGHT, {'cost_per_unit': 1}),
                                                          (((1.0, 0.0), (2.0, 1.0)), COMPUTED_WEIGHT, {'cost_per_unit': 1}),
                                                          (((0.0, 2.0), (2.0, 3.0)), COMPUTED_WEIGHT, {'cost_per_unit': 3}),
                                                          (((2.0, 1.0), (2.0, 3.0)), COMPUTED_WEIGHT, {'cost_per_unit': 1})),
                                        distance_function=manhattan_distance, edge_weight_function=length_cost_per_unit)

# Load a bigger graph from a pickle file.
with open('python/data/graph.pickle', 'rb') as pickled_sample_larger_graph_file:
    larger_graph = pickle.load(pickled_sample_larger_graph_file)
    larger_graph.heuristic_distance_function = manhattan_distance
    larger_graph.edge_weight_function = length_cost_per_unit

# Call the A* function.
toy_a_star_path = a_star(graph=toy_graph, start_node_index=0, goal_node_index=4)
print(f'Toy A* Path:')
pprint(toy_a_star_path)
larger_a_star_path = a_star(graph=larger_graph, start_node_index=0, goal_node_index=4)
print()
print(f'Large A* Path:')
pprint(larger_a_star_path)
print()

# Call the Prim function.
toy_minimum_spanning_tree = prim(graph=toy_graph, start_node_index=0)
print('Toy Prim Minimum Spanning Tree:')
print(toy_minimum_spanning_tree)
print()
larger_minimum_spanning_tree = prim(graph=larger_graph, start_node_index=0)
print('Prim Minimum Spanning Tree:')
print(larger_minimum_spanning_tree)