// Edge weight functions.

use ordered_float::OrderedFloat;

use crate::types::{Edge, Graph, Node, PropertyValue};


// Calculate the cost of an edge based on its unit length and cost-per-unit property.
pub fn length_cost_per_unit(graph: &Graph, edge: &Edge) -> OrderedFloat<f64>
{
    let cost_per_unit: OrderedFloat<f64> = match edge.properties.get("cost_per_unit")
    {
        Some(PropertyValue::Float(cost_per_unit)) => cost_per_unit.clone(),
        _ => OrderedFloat(1.0),
    };
    let attached_nodes = edge.node_indices.iter().map(|index| &graph.nodes[*index]).collect::<Vec<&Node>>();
    cost_per_unit * (graph.distance_function)(attached_nodes[0], attached_nodes[1])
}
