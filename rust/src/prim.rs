// A module containing an implementation of Prim's algorithm for finding the minimum spanning tree of a graph.

use std::collections::{HashSet, BTreeSet};

use crate::types::{Graph, Edge, NoNavigablePathError};



#[allow(dead_code)]
pub fn edge_is_always_valid(_: &Graph, _: &Edge) -> bool
{
    true
}


#[allow(dead_code)]
pub enum ValidEdgeFunction
{
    Specified(fn(&Graph, &Edge) -> bool),
    AlwaysValid,
}


pub fn prim(graph: &Graph, start_node_index: usize, edge_validation_function: ValidEdgeFunction) -> Result<HashSet<usize>, NoNavigablePathError>
{
    let edge_is_valid = match edge_validation_function
    {
        ValidEdgeFunction::Specified(f) => f,
        ValidEdgeFunction::AlwaysValid => edge_is_always_valid,
    };

    let mut included_node_indices = HashSet::new();
    included_node_indices.insert(start_node_index);
    let mut excluded_node_indices: BTreeSet<_> = (0..graph.nodes.len()).collect();
    excluded_node_indices.remove(&start_node_index);

    let mut included_edge_indices = HashSet::new();
    let mut excluded_edge_indices = graph.edge_indices();

    while !excluded_node_indices.is_empty()
    {
        let mut candidate_edges = Vec::new();

        for edge_index in &excluded_edge_indices
        {
            let edge = &graph.edges[*edge_index];

            // Check if the edge connects to the current spanning tree.
            if edge.node_indices.intersection(&included_node_indices).next().is_some()
            {
                candidate_edges.push(edge.clone());
            }
        }

        // Sort the candidate edges by weight.
        candidate_edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());

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
                let indices = edge.node_indices.difference(&included_node_indices).collect::<HashSet<_>>();
                if indices.len() != 1
                {
                    // We've discovered a cycle.  Remove the edge from consideration, and move on.
                    excluded_edge_indices.remove(&graph.edges.iter().position(|e| e == &edge).unwrap());
                    continue;
                }
                let new_node_index = **indices.iter().next().unwrap();

                included_node_indices.insert(new_node_index);
                excluded_node_indices.remove(&new_node_index);
                let edge_index = graph.edges.iter().position(|e| e == &edge).unwrap();
                included_edge_indices.insert(edge_index);
                excluded_edge_indices.remove(&edge_index);
            },
            None =>
            {
                return Err(NoNavigablePathError::new(graph.nodes[start_node_index].clone(), None)); // No goal node in Prim's context.
            }
        }
    }

    let included_edges: Vec<Edge> = graph.edges_from_indices(&included_edge_indices);
    let included_edge_indices = graph.indices_from_edges(&included_edges);
    Ok(included_edge_indices)
}



#[cfg(test)]
mod tests
{
    use super::*;
    use std::collections::{HashSet};
    use crate::test_fixtures::tests::{valid_graph1, valid_graph_from_edge_definitions};

    #[test]
    fn test_prim_finds_minimum_spanning_tree()
    {
        let expected = HashSet::from([0, 1, 2, 4]);
        let result = prim(&valid_graph1(), 0, ValidEdgeFunction::AlwaysValid).unwrap();

        assert_eq!(result.len(), 4);
        assert_eq!(result, expected);
        assert_eq!(result.iter().map(|index| valid_graph1().edges[*index].weight.into_inner()).sum::<f64>(), 5.0);
    }

    #[test]
    fn test_prim_finds_minimum_spanning_tree_from_edge_definitions()
    {
        let expected = HashSet::from([0, 1, 2, 4]);
        let result = prim(&valid_graph_from_edge_definitions(), 0, ValidEdgeFunction::AlwaysValid).unwrap();

        assert_eq!(result.len(), 4);
        assert_eq!(result, expected);
        assert_eq!(result.iter().map(|index| valid_graph_from_edge_definitions().edges[*index].weight.into_inner()).sum::<f64>(), 9.0);
    }
}