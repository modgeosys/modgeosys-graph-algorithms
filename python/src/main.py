"""Usage example(s)."""

import pickle
from pprint import pprint

from modgeosys.graph.types import Node, Edge, Graph
from modgeosys.graph.distance import manhattan_distance, euclidean_distance

from modgeosys.graph.a_star import a_star
from modgeosys.graph.prim import prim

# Define a toy graph.
nodes = [Node(coordinates=(0.0, 0.0)),
         Node(coordinates=(0.0, 2.0)),
         Node(coordinates=(1.0, 0.0)),
         Node(coordinates=(2.0, 1.0)),
         Node(coordinates=(2.0, 3.0))]
edges = (Edge(weight=2.0, node_indices=frozenset((0, 1))),
         Edge(weight=1.0, node_indices=frozenset((0, 2))),
         Edge(weight=1.0, node_indices=frozenset((2, 3))),
         Edge(weight=3.0, node_indices=frozenset((1, 4))),
         Edge(weight=1.0, node_indices=frozenset((3, 4))))
toy_graph = Graph(nodes=nodes, edges=edges)

# Load a bigger graph from a pickle file.
with open('python/data/graph.pickle', 'rb') as f:
    larger_graph = pickle.load(f)

# Call the A* function.
path = a_star(graph=toy_graph, start_node_index=0, goal_node_index=4, heuristic_distance=manhattan_distance)
print(f'A* Path:')
pprint(path)
print()

# Call the Prim function.
minimum_spanning_tree = prim(graph=larger_graph, start_node_index=0)
print('Prim Minimum Spanning Tree:')
print(minimum_spanning_tree)