# modgeosys-graph-algorithms: Graph Algorithms

A repository for [hopefully] clean, readable, and easily-called implementations of some navigation,
path planning, and obstacle avoidance algorithms I will be using in the near future, written in modern
Python and/or Rust with Python bindings. I'll be adding more algorithm implementations over time.

## Algorithms: Currently implemented + planned
* [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) - Graph path search algorithm.
  * Code-complete in both Python and Rust.
  * Needs a more thorough test suite.
  * Needs Python bindings for Rust implementation.
* [Prim's algorithm](https://en.wikipedia.org/wiki/Prim's_algorithm) - Prim's Minimum Spanning Tree algorithm.
  * Code-complete in Python.
  * Tested on toy dataset in test suite.
  * Tested on larger sample (pickled) dataset, not yet incorporated into test suite.
  * Needs a Rust implementation and corresponding Python bindings.

## Usage

### A\*

#### Python

```python
import pickle
from pprint import pprint

from modgeosys.graph.types import Graph
from modgeosys.graph.distance import manhattan_distance, euclidean_distance
from modgeosys.graph.a_star import a_star

# Define a toy graph.
toy_graph = Graph.from_edge_definitions(((2, ((0.0, 0.0), (0.0, 2.0))),
                                         (1, ((0.0, 0.0), (1.0, 0.0))),
                                         (1, ((1.0, 0.0), (2.0, 1.0))),
                                         (3, ((0.0, 2.0), (2.0, 3.0))),
                                         (1, ((2.0, 1.0), (2.0, 3.0)))))

# Load a bigger graph from a pickle file.
with open('python/data/graph.pickle', 'rb') as pickled_sample_larger_graph_file:
    larger_graph = pickle.load(pickled_sample_larger_graph_file)

# Call the A* function.
toy_a_star_path = a_star(graph=toy_graph, start_node_index=0, goal_node_index=4, heuristic_distance=manhattan_distance)
print(f'Toy A* Path:')
pprint(toy_a_star_path)
print()
larger_a_star_path = a_star(graph=larger_graph, start_node_index=0, goal_node_index=4, heuristic_distance=manhattan_distance)
print(f'Large A* Path:')
pprint(larger_a_star_path)
```

#### Rust
```rust
use std::collections::HashSet;

use modgeosys_graph::a_star::a_star;
use modgeosys_graph::types::{Node, Edge, Graph};
use modgeosys_graph::distance::manhattan_distance;

fn main()
{
  // Define a graph.
  let nodes = vec![Node::new(vec![0.0, 0.0]),
                   Node::new(vec![0.0, 2.0]),
                   Node::new(vec![1.0, 0.0]),
                   Node::new(vec![2.0, 1.0]),
                   Node::new(vec![2.0, 3.0])];
  let edges = vec![Edge::new(2.0, HashSet::from([0, 1])),
                   Edge::new(1.0, HashSet::from([0, 2])),
                   Edge::new(1.0, HashSet::from([2, 3])),
                   Edge::new(3.0, HashSet::from([1, 4])),
                   Edge::new(1.0, HashSet::from([3, 4]))];
  let graph = Graph::new(nodes, edges);

  // Call the A* function.
  let path = a_star(&graph, 0, 4, manhattan_distance).unwrap();

  // Report the resulting path.
  println!("{:?}", path);
}
```

### Prim's algorithm

#### Python

```python
import pickle

from modgeosys.graph.types import Graph
from modgeosys.graph.prim import prim

# Define a toy graph.
toy_graph = Graph.from_edge_definitions(((2, ((0.0, 0.0), (0.0, 2.0))),
                                         (1, ((0.0, 0.0), (1.0, 0.0))),
                                         (1, ((1.0, 0.0), (2.0, 1.0))),
                                         (3, ((0.0, 2.0), (2.0, 3.0))),
                                         (1, ((2.0, 1.0), (2.0, 3.0)))))

# Load a bigger graph from a pickle file.
with open('python/data/graph.pickle', 'rb') as pickled_sample_larger_graph_file:
    larger_graph = pickle.load(pickled_sample_larger_graph_file)

# Call the Prim function.
toy_minimum_spanning_tree = prim(graph=toy_graph, start_node_index=0)
print('Toy Prim Minimum Spanning Tree:')
print(toy_minimum_spanning_tree)
print()
larger_minimum_spanning_tree = prim(graph=larger_graph, start_node_index=0)
print('Prim Minimum Spanning Tree:')
print(larger_minimum_spanning_tree)
```