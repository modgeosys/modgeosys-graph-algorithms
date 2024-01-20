Draft REA# nav-algorithms: Navigation Algorithms

A repository for [hopefully] clean, readable, and easily-called implementations of some navigation,
path planning, and obstacle avoidance algorithms I will be using in the near future, written in modern
Python and/or Rust with Python bindings. I'll be adding more algorithm implementations over time.

## Algorithms: Currently implemented + planned
* [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) - A* graph path search algorithm.
  * code-complete in Python.
  * Needs a more thorough test suite.
  * Needs a Rust implementation.
* Probabilistic Roadmap (PRM).
  * Planned.

## Usage

### A* (Python)
```python
from modgeosys.nav.a_star import a_star
from modgeosys.nav.types import Edge, Graph
from modgeosys.nav.distance import manhattan_distance, euclidean_distance

# Define a graph.
nodes = [(0, 0), (0, 2), (1, 0), (2, 1), (2, 3)]
edges = (Edge(weight=2, node_indices=frozenset((0, 1))),
         Edge(weight=1, node_indices=frozenset((0, 2))),
         Edge(weight=1, node_indices=frozenset((2, 3))),
         Edge(weight=3, node_indices=frozenset((1, 4))),
         Edge(weight=1, node_indices=frozenset((3, 4))))
graph = Graph(nodes=nodes, edges=edges)

# Call the A* function.
path = a_star(graph=graph, start_node_index=0, goal_node_index=4, heuristic_distance=manhattan_distance)

# Report the resulting path.
print(path)
```