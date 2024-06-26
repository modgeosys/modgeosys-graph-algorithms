# modgeosys-graph-algorithms: Spatial Graph Algorithms

A repository for [hopefully] clean, readable, and easily-called implementations of some spatial navigation,
path planning, and obstacle avoidance algorithms I will be using in the near future, written in modern
Python and/or Rust with Python bindings. I'll be adding more algorithm implementations over time.

## Algorithms: Currently implemented + planned
* [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) - Graph path search algorithm.
  * Code-complete in both Python and Rust.
  * Needs a more thorough test suite.
  * Python tested on larger sample (pickled) dataset.
  * Needs Python bindings for Rust implementation.
* [Prim's algorithm](https://en.wikipedia.org/wiki/Prim's_algorithm) - Prim's Minimum Spanning Tree algorithm.
  * Code-complete in both Python and Rust.
  * Tested on toy dataset in test suite.
  * Python tested on larger sample (pickled) dataset.
  * Needs Python bindings for Rust implementation.

## Usage

### A\*

#### Python

```python
import pickle
from pprint import pprint

from modgeosys.graph.edge_weight import length_cost_per_unit
from modgeosys.graph.types import Graph, COMPUTED_WEIGHT
from modgeosys.graph.distance import manhattan_distance, euclidean_distance
from modgeosys.graph.a_star import a_star

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
  larger_graph.distance_function = manhattan_distance
  larger_graph.edge_weight_function = length_cost_per_unit

# Call the A* function.
toy_a_star_path = a_star(graph=toy_graph, start_node_index=0, goal_node_index=4)
print('Toy A* Path:')
pprint(toy_a_star_path)
print()
larger_a_star_path = a_star(graph=larger_graph, start_node_index=0, goal_node_index=4)
print('Large A* Path:')
pprint(larger_a_star_path)
```

#### Rust
```rust
use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use modgeosys_graph::types::{PropertyValue, EdgeDefinition, Graph, WeightOption};
use modgeosys_graph::edge_weight::length_cost_per_unit;
use modgeosys_graph::distance::manhattan_distance;
use modgeosys_graph::a_star::a_star;



fn main()
{
  // Define a graph.
  let toy_graph = Graph::from_edge_definitions(vec![EdgeDefinition(vec![vec![0.0, 0.0], vec![0.0, 2.0]],
                                                                   WeightOption::Computed,
                                                                   [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(2.0)))].iter().cloned().collect()),
                                                    EdgeDefinition(vec![vec![0.0, 0.0], vec![1.0, 0.0]],
                                                                   WeightOption::Computed,
                                                                   [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                                                    EdgeDefinition(vec![vec![1.0, 0.0], vec![2.0, 1.0]],
                                                                   WeightOption::Computed,
                                                                   [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                                                    EdgeDefinition(vec![vec![0.0, 2.0], vec![2.0, 3.0]],
                                                                   WeightOption::Computed,
                                                                   [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(3.0)))].iter().cloned().collect()),
                                                    EdgeDefinition(vec![vec![2.0, 1.0], vec![2.0, 3.0]],
                                                                   WeightOption::Computed,
                                                                   [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect())],
                                               BTreeMap::new(), manhattan_distance, Some(length_cost_per_unit));

  // Call the A* function.
  let toy_a_star_path = a_star(&toy_graph, 0, 4).unwrap();
  println!("{:?}", toy_a_star_path);
}
```

### Prim's algorithm

#### Python

```python
import pickle

from modgeosys.graph.types import Graph, COMPUTED_WEIGHT
from modgeosys.graph.edge_weight import length_cost_per_unit
from modgeosys.graph.distance import manhattan_distance, euclidean_distance
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

# Call the Prim function.
toy_minimum_spanning_tree = prim(graph=toy_graph, start_node_index=0)
print('Toy Prim Minimum Spanning Tree:')
print(toy_minimum_spanning_tree)
print()
larger_minimum_spanning_tree = prim(graph=larger_graph, start_node_index=0)
print('Prim Minimum Spanning Tree:')
print(larger_minimum_spanning_tree)
```

#### Rust
```rust
use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use modgeosys_graph::edge_weight::length_cost_per_unit;
use modgeosys_graph::types::{PropertyValue, EdgeDefinition, Graph, WeightOption};
use modgeosys_graph::distance::manhattan_distance;
use modgeosys_graph::prim::{prim, ValidEdgeFunction};



fn main()
{
  // Define a graph.
  let toy_graph = Graph::from_edge_definitions(vec![EdgeDefinition(vec![vec![0.0, 0.0], vec![0.0, 2.0]],
                                                                   WeightOption::Computed,
                                                                   [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(2.0)))].iter().cloned().collect()),
                                                    EdgeDefinition(vec![vec![0.0, 0.0], vec![1.0, 0.0]],
                                                                   WeightOption::Computed,
                                                                   [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                                                    EdgeDefinition(vec![vec![1.0, 0.0], vec![2.0, 1.0]],
                                                                   WeightOption::Computed,
                                                                   [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect()),
                                                    EdgeDefinition(vec![vec![0.0, 2.0], vec![2.0, 3.0]],
                                                                   WeightOption::Computed,
                                                                   [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(3.0)))].iter().cloned().collect()),
                                                    EdgeDefinition(vec![vec![2.0, 1.0], vec![2.0, 3.0]],
                                                                   WeightOption::Computed,
                                                                   [("cost_per_unit".to_string(), PropertyValue::Float(OrderedFloat(1.0)))].iter().cloned().collect())],
                                               BTreeMap::new(), manhattan_distance, Some(length_cost_per_unit));

  // Call the Prim function.
  let toy_minimum_spanning_tree = prim(&toy_graph, 0, ValidEdgeFunction::AlwaysValid).unwrap();
  println!("Toy Minimum Spanning Tree:");
  println!("{:?}", toy_graph.edges_from_indices(&toy_minimum_spanning_tree));
}
```
