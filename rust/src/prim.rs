// A module containing an implementation of Prim's algorithm for finding the minimum spanning tree of a graph.

use std::collections::{HashSet, BTreeSet};

use crate::types::{Graph, Edge, NoNavigablePathError};




pub fn edge_is_always_valid(_: &Graph, _: &Edge) -> bool
{
    true
}


enum ValidEdgeFunction
{
    Specified(fn(&Graph, &Edge) -> bool),
    AlwaysValid,
}


pub fn prim(graph: &Graph, start_node_index: usize, edge_validation_function: ValidEdgeFunction) -> Result<HashSet<Edge>, NoNavigablePathError>
{
    let edge_is_valid = match edge_validation_function
    {
        ValidEdgeFunction::Specified(f) => f,
        ValidEdgeFunction::AlwaysValid => edge_is_always_valid,
    };

    let nodes_count = graph.nodes.len();
    let mut included_node_indices = HashSet::new();
    included_node_indices.insert(start_node_index);
    let mut excluded_node_indices: BTreeSet<_> = (0..nodes_count).collect();
    excluded_node_indices.remove(&start_node_index);

    let mut included_edges = HashSet::new();
    let mut excluded_edges: HashSet<_> = graph.edges.iter().cloned().collect();

    while !excluded_node_indices.is_empty()
    {
        let mut candidate_edges = Vec::new();

        for edge in &excluded_edges
        {
            // Check if the edge connects to the current spanning tree.
            if edge.node_indices.intersection(&included_node_indices).next().is_some()
            {
                candidate_edges.push(edge.clone());
            }
        }

        let mut best_edge = None;

        for edge in candidate_edges
        {
            // Assuming edge validation is taken care of within the `Edge` structure or by custom logic outside this snippet.
            if edge_is_valid(graph, &edge)
            {
                best_edge = Some(edge);
                break;
            }
        }

        match best_edge
        {
            Some(edge) =>
            {
                // Update the sets based on the selected edge.
                let new_node_index = *edge.node_indices.difference(&included_node_indices).next().unwrap();
                included_node_indices.insert(new_node_index);
                excluded_node_indices.remove(&new_node_index);
                included_edges.insert(edge.clone());
                excluded_edges.remove(&edge);
            },
            None =>
            {
                return Err(NoNavigablePathError::new(graph.nodes[start_node_index].clone(), None)); // No goal node in Prim's context.
            }
        }
    }

    Ok(included_edges)
}
