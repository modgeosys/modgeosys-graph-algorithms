// Usage example(s).

use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use modgeosys_graph::types::{PropertyValue, EdgeDefinition, Graph, WeightOption};
use modgeosys_graph::edge_weight::length_cost_per_unit;
use modgeosys_graph::distance::manhattan_distance;
use modgeosys_graph::a_star::a_star;
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

    // Call the A* function.
    let toy_a_star_path = a_star(&toy_graph, 0, 4).unwrap();
    println!("{:?}", toy_a_star_path);

    // Call the Prim function.
    let toy_minimum_spanning_tree = prim(&toy_graph, 0, ValidEdgeFunction::AlwaysValid).unwrap();
    println!("Toy Minimum Spanning Tree:");
    println!("{:?}", toy_graph.edges_from_indices(&toy_minimum_spanning_tree));
}
