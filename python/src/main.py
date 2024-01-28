"""Usage example(s)."""

from modgeosys.nav.a_star import a_star
from modgeosys.nav.types import Edge, Graph
from modgeosys.nav.distance import manhattan_distance, euclidean_distance

# Define a graph.
nodes = [(0.0, 0.0), (0.0, 2.0), (1.0, 0.0), (2.0, 1.0), (2.0, 3.0)]
edges = (Edge(weight=2.0, node_indices=frozenset((0, 1))),
         Edge(weight=1.0, node_indices=frozenset((0, 2))),
         Edge(weight=1.0, node_indices=frozenset((2, 3))),
         Edge(weight=3.0, node_indices=frozenset((1, 4))),
         Edge(weight=1.0, node_indices=frozenset((3, 4))))
graph = Graph(nodes=nodes, edges=edges)

# Call the A* function.
path = a_star(graph=graph, start_node_index=0, goal_node_index=4, heuristic_distance=manhattan_distance)

# Report the resulting path.
print(path)
