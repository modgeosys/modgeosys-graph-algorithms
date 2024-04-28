// Usage example(s).

use std::collections::BTreeMap;

use modgeosys_graph::a_star::a_star;
use modgeosys_graph::edge_weight::length_cost_per_unit;
use modgeosys_graph::types::{EdgeDefinition, Graph};
use modgeosys_graph::distance::manhattan_distance;



fn main()
{
    // Define a graph.
    let toy_graph = Graph::from_edge_definitions(vec![EdgeDefinition(vec![vec![0.0, 0.0], vec![0.0, 2.0]], 2.0, BTreeMap::new()),
                                                          EdgeDefinition(vec![vec![0.0, 0.0], vec![1.0, 0.0]], 1.0, BTreeMap::new()),
                                                          EdgeDefinition(vec![vec![1.0, 0.0], vec![2.0, 1.0]], 1.0, BTreeMap::new()),
                                                          EdgeDefinition(vec![vec![0.0, 2.0], vec![2.0, 3.0]], 3.0, BTreeMap::new()),
                                                          EdgeDefinition(vec![vec![2.0, 1.0], vec![2.0, 3.0]], 1.0, BTreeMap::new())],
                                                 BTreeMap::new(), manhattan_distance, length_cost_per_unit);

    // Call the A* function.
    let toy_a_star_path = a_star(&toy_graph, 0, 4).unwrap();

    // Report the resulting path.
    println!("{:?}", toy_a_star_path);
}
