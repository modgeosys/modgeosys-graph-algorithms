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


pub fn prim(graph: &Graph, start_node_index: usize, edge_validation_function: ValidEdgeFunction) -> Result<Vec<Edge>, NoNavigablePathError>
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

    let mut included_edge_indices = HashSet::new();
    let mut excluded_edge_indices: HashSet<_> = graph.edges.iter().enumerate().map(|(index, _)| index).collect();

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

    let included_edges = included_edge_indices.iter().map(|index| graph.edges[*index].clone()).collect();
    Ok(included_edges)
}



#[cfg(test)]
mod tests
{
    use super::*;
    use std::collections::{BTreeMap, HashSet};
    use crate::types::Edge;
    use crate::test_fixtures::tests::{valid_graph1, valid_graph2, valid_graph3};
    use crate::types::WeightOption::Specified;

    #[test]
    fn test_prim_finds_minimum_spanning_tree()
    {
        let expected = vec![Edge::new(HashSet::from([0, 1]), Specified(2.0), BTreeMap::new()),
                            Edge::new(HashSet::from([0, 2]), Specified(1.0), BTreeMap::new()),
                            Edge::new(HashSet::from([2, 3]), Specified(1.0), BTreeMap::new()),
                            Edge::new(HashSet::from([3, 4]), Specified(1.0), BTreeMap::new())];
        let result = prim(&valid_graph1(), 0, ValidEdgeFunction::AlwaysValid).unwrap();

        assert_eq!(result.len(), 4);
        assert_eq!(result, expected);
        assert_eq!(result.iter().map(|edge| edge.weight.into_inner()).sum::<f64>(), 5.0);
    }
}